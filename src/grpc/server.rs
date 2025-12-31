use crate::clients::llama::LlamaClient;
use sentiric_contracts::sentiric::llm::v1::llm_gateway_service_server::LlmGatewayService;
use sentiric_contracts::sentiric::llm::v1::{
    GenerateDialogStreamRequest, GenerateDialogStreamResponse,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::{info, warn, instrument};
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
        
        // 1. TRACE ID EXTRACT (Metaveriyi Çek)
        let trace_id = request.metadata()
            .get("x-trace-id")
            .and_then(|m| m.to_str().ok())
            .map(|s| s.to_string());

        let req = request.into_inner();
        let model_selector = req.model_selector.clone();
        
        // [FIX] Selector ismini düzelt
        let effective_selector = if model_selector == "local" { "llama" } else { &model_selector };
        
        info!(
            "LLM Request received. Selector: {}, TraceID: {}", 
            effective_selector, // local yerine llama yazacak
            trace_id.as_deref().unwrap_or("none")
        );


        if model_selector != "local" && !model_selector.is_empty() {
             warn!("Requested model '{}' not explicitly supported yet, falling back to Local Llama.", model_selector);
        }

        let llama_req = req.llama_request.ok_or_else(|| 
            Status::invalid_argument("llama_request field is required inside GenerateDialogStreamRequest")
        )?;

        // 2. TRACE ID PROPAGATION (İlet)
        let mut upstream_stream = self.llama_client.generate_stream(llama_req, trace_id).await?;

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
                        let _ = tx.send(Err(e)).await;
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}