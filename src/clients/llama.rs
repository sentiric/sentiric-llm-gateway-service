// Dosya: src/clients/llama.rs
use crate::config::AppConfig;
use crate::tls::load_client_tls_config;
use sentiric_contracts::sentiric::llm::v1::llama_service_client::LlamaServiceClient;
use sentiric_contracts::sentiric::llm::v1::{GenerateStreamRequest, GenerateStreamResponse};
use tonic::transport::{Channel, Endpoint};
use tonic::Request;
use tonic::metadata::MetadataValue;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{info, error};

#[derive(Clone)]
pub struct LlamaClient {
    client: LlamaServiceClient<Channel>,
}

impl LlamaClient {
    pub async fn connect(config: &Arc<AppConfig>) -> anyhow::Result<Self> {
        let url = config.llm_llama_service_grpc_url.clone();
        
        // [ARCH-COMPLIANCE] constraints.yaml: grpc_communication ZORUNLU OLARAK mTLS ile şifrelenmelidir. Insecure bağlantı kabul edilemez.
        if url.starts_with("http://") {
            panic!("⚠️ [ARCH-COMPLIANCE] Insecure connection to {} is FORBIDDEN. Use https:// with mTLS.", url);
        }

        info!("🔐 Connecting to Llama Engine (mTLS): {}", url);
        let tls_config = load_client_tls_config(config).await?;
        let channel = Endpoint::from_shared(url)?.tls_config(tls_config)?.connect().await?;

        Ok(Self {
            client: LlamaServiceClient::new(channel),
        })
    }

    pub async fn generate_stream(
        &self,
        request: GenerateStreamRequest,
        trace_id: Option<String>,
    ) -> Result<tonic::Streaming<GenerateStreamResponse>, tonic::Status> {
        let mut client = self.client.clone();
        let mut req = Request::new(request);

        if let Some(tid) = trace_id {
            if let Ok(meta_val) = MetadataValue::from_str(&tid) {
                req.metadata_mut().insert("x-trace-id", meta_val);
            }
        }

        match client.generate_stream(req).await {
            Ok(response) => Ok(response.into_inner()),
            Err(e) => {
                error!("❌ Llama Engine gRPC call failed: {}", e);
                Err(e)
            }
        }
    }

    // Basit bağlantı kontrolü
    pub fn is_ready(&self) -> bool {
        true 
    }
}