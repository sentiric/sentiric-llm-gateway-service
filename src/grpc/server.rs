// Dosya: src/grpc/server.rs
use crate::clients::llama::LlamaClient;
use futures::StreamExt;
use sentiric_contracts::sentiric::llm::v1::llm_gateway_service_server::LlmGatewayService;
use sentiric_contracts::sentiric::llm::v1::{
    GenerateDialogStreamRequest, GenerateDialogStreamResponse,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

pub struct LlmGateway {
    llama_client: LlamaClient,
}

impl LlmGateway {
    pub fn new(llama_client: LlamaClient) -> Self {
        Self { llama_client }
    }
}

#[tonic::async_trait]
impl LlmGatewayService for LlmGateway {
    type GenerateDialogStreamStream = ReceiverStream<Result<GenerateDialogStreamResponse, Status>>;

    #[instrument(skip(self, request))]
    async fn generate_dialog_stream(
        &self,
        request: Request<GenerateDialogStreamRequest>,
    ) -> Result<Response<Self::GenerateDialogStreamStream>, Status> {
        // 1. TRACE & SPAN ID EXTRACT (SUTS v4.0 Compliance)
        let trace_id_opt = request
            .metadata()
            .get("x-trace-id")
            .and_then(|m| m.to_str().ok())
            .map(|s| s.to_string());

        let span_id_opt = request
            .metadata()
            .get("x-span-id")
            .and_then(|m| m.to_str().ok())
            .map(|s| s.to_string());

        // Loglama için saf string halleri
        let tid_log = trace_id_opt.as_deref().unwrap_or("unknown");
        let sid_log = span_id_opt.as_deref().unwrap_or("unknown");

        let req = request.into_inner();
        let tenant_id = req.tenant_id.clone();

        // [ARCH-COMPLIANCE] Strict Tenant Isolation Check
        if tenant_id.is_empty() {
            error!(
                event = "MISSING_TENANT_ID",
                trace_id = %tid_log,
                span_id = %sid_log,
                "Tenant ID is missing in request. Request rejected."
            );
            return Err(Status::invalid_argument(
                "tenant_id is strictly required for isolation",
            ));
        }

        let model_selector = req.model_selector.clone();
        let effective_selector = if model_selector == "local" {
            "llama"
        } else {
            &model_selector
        };

        info!(
            event = "LLM_REQUEST_RECEIVED",
            trace_id = %tid_log,
            span_id = %sid_log,
            tenant_id = %tenant_id,
            selector = %effective_selector,
            "LLM Request received."
        );

        let llama_req = req.llama_request.ok_or_else(|| {
            error!(
                event = "INVALID_REQUEST",
                trace_id = %tid_log,
                span_id = %sid_log,
                tenant_id = %tenant_id,
                "llama_request field missing"
            );
            Status::invalid_argument(
                "llama_request field is required inside GenerateDialogStreamRequest",
            )
        })?;

        // 2. UPSTREAM CALL (Orijinal Option'ları gönderiyoruz)
        let mut upstream_stream = self
            .llama_client
            .generate_stream(
                llama_req,
                trace_id_opt.clone(),
                span_id_opt.clone(),
                Some(tenant_id.clone()),
            )
            .await?;

        let (tx, rx) = tokio::sync::mpsc::channel(128);
        let tid_thread = tid_log.to_string();
        let sid_thread = sid_log.to_string();
        let ten_thread = tenant_id.clone();

        tokio::spawn(async move {
            while let Some(result) = upstream_stream.next().await {
                match result {
                    Ok(llama_resp) => {
                        let gateway_resp = GenerateDialogStreamResponse {
                            llama_response: Some(llama_resp),
                        };
                        if tx.send(Ok(gateway_resp)).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        error!(
                            event = "LLM_STREAM_ERROR",
                            trace_id = %tid_thread,
                            span_id = %sid_thread,
                            tenant_id = %ten_thread,
                            error = %e,
                            "Upstream streaming error occurred."
                        );
                        let _ = tx.send(Err(e)).await;
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
