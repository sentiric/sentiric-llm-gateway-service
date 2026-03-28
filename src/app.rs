// Dosya: src/app.rs
use crate::config::AppConfig;
use crate::clients::llama::LlamaClient;
use crate::grpc::server::LlmGateway;
use crate::tls::load_server_tls_config;
use crate::metrics::start_metrics_server;
use sentiric_contracts::sentiric::llm::v1::llm_gateway_service_server::LlmGatewayServiceServer;
use tonic::transport::Server;
use std::net::SocketAddr;
use tracing::{info, error};
use anyhow::{Result, Context};
use std::sync::Arc;

pub struct App;

impl App {
    pub async fn run() -> Result<()> {
        let config = Arc::new(AppConfig::load()?);
        
        // [ARCH-COMPLIANCE] constraints.yaml: logging_format ZORUNLU OLARAK JSON formatında olmalıdır.
        tracing_subscriber::fmt().with_env_filter(&config.rust_log).json().init();
        
        //[ARCH-COMPLIANCE] SUTS v4.0: event tag added
        info!(
            event = "SERVICE_START",
            schema_v = "1.0.0",
            service_version = %config.service_version,
            "🚀 LLM Gateway Service starting..."
        );

        // 1. Upstream Bağlantısı
        let llama_client = LlamaClient::connect(&config).await?;
        
        // 2. Metrics & Health Server Başlatma
        let metrics_addr: SocketAddr = format!("{}:{}", config.host, config.http_port).parse()?; // Port 16020
        start_metrics_server(metrics_addr, llama_client.clone());

        // 3. gRPC Server Başlatma
        let addr: SocketAddr = format!("{}:{}", config.host, config.grpc_port).parse()?;
        let gateway_service = LlmGateway::new(llama_client);
        
        let mut builder = Server::builder();

        // [ARCH-COMPLIANCE] constraints.yaml: grpc_communication ZORUNLU OLARAK mTLS ile şifrelenmelidir (Fail Fast).
        if config.llm_gateway_service_cert_path.is_empty() || config.grpc_tls_ca_path.is_empty() {
            panic!("⚠️ [ARCH-COMPLIANCE] TLS paths empty. Starting in INSECURE mode is FORBIDDEN.");
        }

        let tls = load_server_tls_config(&config).await
            .context("⚠️ [ARCH-COMPLIANCE] TLS Load Failed. Insecure fallback is FORBIDDEN.")?;
            
        builder = builder.tls_config(tls)?;
        
        info!(
            event = "GRPC_SERVER_READY", 
            address = %addr, 
            "🎧 gRPC Server listening (mTLS Enabled)"
        );

        if let Err(e) = builder
            .add_service(LlmGatewayServiceServer::new(gateway_service))
            .serve(addr)
            .await 
        {
            error!(event = "GRPC_SERVER_CRASH", error = %e, "gRPC Server stopped unexpectedly.");
            return Err(e.into());
        }

        Ok(())
    }
}