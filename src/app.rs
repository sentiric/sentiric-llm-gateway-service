use crate::config::AppConfig;
use crate::clients::llama::LlamaClient;
use crate::grpc::server::LlmGateway;
use crate::tls::load_server_tls_config;
use sentiric_contracts::sentiric::llm::v1::llm_gateway_service_server::LlmGatewayServiceServer;
use tonic::transport::Server;
use std::net::SocketAddr;
use tracing::info;
use anyhow::Result;
use std::sync::Arc;

pub struct App;

impl App {
    pub async fn run() -> Result<()> {
        // 1. Config Yükle
        let config = Arc::new(AppConfig::load()?);

        // 2. Loglama Başlat
        tracing_subscriber::fmt()
            .with_env_filter(&config.rust_log)
            .init();

        info!("🚀 LLM Gateway Service v{} başlatılıyor...", config.service_version);

        // 3. Upstream Client Bağlantısı
        let llama_client = LlamaClient::connect(&config).await?;

        // 4. Server Hazırlığı
        let addr: SocketAddr = format!("{}:{}", config.host, config.grpc_port).parse()?;
        let gateway_service = LlmGateway::new(llama_client);
        
        // 5. TLS Config Yükle
        let tls_config = load_server_tls_config(&config).await?;

        info!("🎧 gRPC Server listening on {} (mTLS Enabled)", addr);

        Server::builder()
            .tls_config(tls_config)?
            .add_service(LlmGatewayServiceServer::new(gateway_service))
            .serve(addr)
            .await?;

        Ok(())
    }
}