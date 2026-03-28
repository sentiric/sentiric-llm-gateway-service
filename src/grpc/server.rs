// Dosya: src/grpc/server.rs
use crate::clients::llama::LlamaClient;
use sentiric_contracts::sentiric::llm::v1::llm_gateway_service_server::LlmGatewayService;
use sentiric_contracts::sentiric::llm::v1::{
    GenerateDialogStreamRequest, GenerateDialogStreamResponse,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::{info, warn, error, instrument};
use futures::StreamExt;

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
        
        // 1. TRACE & SPAN ID EXTRACT (Metaveriyi Çek - SUTS v4.0 Compliance)
        let trace_id = request.metadata()
            .get("x-trace-id")
            .and_then(|m| m.to_str().ok())
            .map(|s| s.to_string());
            
        let span_id = request.metadata()
            .get("x-span-id")
            .and_then(|m| m.to_str().ok())
            .map(|s| s.to_string());

        let req = request.into_inner();
        let tenant_id = req.tenant_id.clone();
        
        //[ARCH-COMPLIANCE] Strict Tenant Isolation Check
        if tenant_id.is_empty() {
            error!(
                event = "MISSING_TENANT_ID",
                trace_id = ?trace_id,
                span_id = ?span_id,
                "Tenant ID is missing in request. Request rejected."
            );
            return Err(Status::invalid_argument("tenant_id is strictly required for isolation"));
        }

        let model_selector = req.model_selector.clone();
        let effective_selector = if model_selector == "local" { "llama" } else { &model_selector };
        
        info!(
            event = "LLM_REQUEST_RECEIVED",
            trace_id = ?trace_id,
            span_id = ?span_id,
            tenant_id = %tenant_id,
            selector = %effective_selector,
            "LLM Request received."
        );

        if model_selector != "local" && !model_selector.is_empty() {
             warn!(
                 event = "UNSUPPORTED_MODEL_FALLBACK",
                 trace_id = ?trace_id,
                 span_id = ?span_id,
                 tenant_id = %tenant_id,
                 requested_model = %model_selector,
                 "Requested model not explicitly supported yet, falling back to Local Llama."
             );
        }

        let llama_req = req.llama_request.ok_or_else(|| {
            error!(
                event = "INVALID_REQUEST", 
                trace_id = ?trace_id, 
                span_id = ?span_id, 
                tenant_id = %tenant_id, 
                "llama_request field missing"
            );
            Status::invalid_argument("llama_request field is required inside GenerateDialogStreamRequest")
        })?;

        // 2. TRACE ID PROPAGATION (İlet)
        let mut upstream_stream = self.llama_client.generate_stream(
            llama_req, 
            trace_id.clone(), 
            span_id.clone(), 
            Some(tenant_id.clone())
        ).await?;

        let (tx, rx) = tokio::sync::mpsc::channel(128);

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
                        //[ARCH-COMPLIANCE] Surgical Folding Support
                        error!(
                            event = "LLM_STREAM_ERROR",
                            trace_id = ?trace_id,
                            span_id = ?span_id,
                            tenant_id = %tenant_id,
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