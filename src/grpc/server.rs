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
        let req = request.into_inner();
        let model_selector = req.model_selector.clone();
        
        info!("LLM Request received. Selector: {}, Tenant: {}", model_selector, req.tenant_id);

        // 1. ROUTING LOGIC (Basit Başlangıç)
        // Eğer selector "local" ise veya boşsa Llama'ya git.
        // Gelecekte "gemini" veya "gpt" gelirse burada switch-case olacak.
        
        if model_selector != "local" && !model_selector.is_empty() {
             warn!("Requested model '{}' not explicitly supported yet, falling back to Local Llama.", model_selector);
        }

        // 2. Request Mapping (Gateway Request -> Llama Request)
        // Gateway proto'sundaki `llama_request` alanını (field 10) alıyoruz.
        let llama_req = req.llama_request.ok_or_else(|| 
            Status::invalid_argument("llama_request field is required inside GenerateDialogStreamRequest")
        )?;

        // 3. Upstream Call
        let mut upstream_stream = self.llama_client.generate_stream(llama_req).await?;

        // 4. Response Mapping (Stream Transformation)
        let (tx, rx) = tokio::sync::mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(result) = upstream_stream.next().await {
                match result {
                    Ok(llama_resp) => {
                        // Llama yanıtını Gateway yanıtına sarıyoruz
                        let gateway_resp = GenerateDialogStreamResponse {
                            llama_response: Some(llama_resp),
                        };
                        
                        if tx.send(Ok(gateway_resp)).await.is_err() {
                            break; // İstemci koptu
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