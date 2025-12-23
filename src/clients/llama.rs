use crate::config::AppConfig;
use crate::tls::load_client_tls_config;
use sentiric_contracts::sentiric::llm::v1::llama_service_client::LlamaServiceClient;
use sentiric_contracts::sentiric::llm::v1::{GenerateStreamRequest, GenerateStreamResponse};
use tonic::transport::{Channel, Endpoint};
use tonic::Request;
use std::sync::Arc;
use tracing::{info, error};

#[derive(Clone)]
pub struct LlamaClient {
    client: LlamaServiceClient<Channel>,
}

impl LlamaClient {
    pub async fn connect(config: &Arc<AppConfig>) -> anyhow::Result<Self> {
        let url = config.llm_llama_url.clone();
        info!("Connecting to Llama Engine at: {}", url);

        // mTLS Konfigürasyonunu Yükle
        let tls_config = load_client_tls_config(config).await?;

        // Endpoint oluştur ve bağlan
        let channel = Endpoint::from_shared(url)?
            .tls_config(tls_config)?
            .connect()
            .await?;

        Ok(Self {
            client: LlamaServiceClient::new(channel),
        })
    }

    pub async fn generate_stream(
        &self,
        request: GenerateStreamRequest,
    ) -> Result<tonic::Streaming<GenerateStreamResponse>, tonic::Status> {
        let mut client = self.client.clone();
        
        // Metadata ekleme (Trace ID vb.) buraya eklenebilir
        let req = Request::new(request);

        match client.generate_stream(req).await {
            Ok(response) => Ok(response.into_inner()),
            Err(e) => {
                error!("Llama Engine gRPC call failed: {}", e);
                Err(e)
            }
        }
    }
}