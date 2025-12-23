use config::{Config, File, Environment};
use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub env: String,
    pub rust_log: String,
    pub service_version: String,
    
    // Server Settings
    pub host: String,
    pub grpc_port: u16,

    // Upstream Services (Expert Engines)
    pub llm_llama_url: String, // http://llm-llama-service:16071

    // Security (mTLS Paths)
    pub grpc_tls_ca_path: String,
    pub llm_gateway_service_cert_path: String,
    pub llm_gateway_service_key_path: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let builder = Config::builder()
            .add_source(File::with_name(".env").required(false))
            .add_source(Environment::default())
            // Defaults
            .set_default("env", "development")?
            .set_default("rust_log", "info")?
            .set_default("service_version", "1.0.0")?
            .set_default("host", "0.0.0.0")?
            .set_default("grpc_port", 16021)?
            // Default to local docker compose service name
            .set_default("llm_llama_url", "http://llm-llama-service:16071")?;

        builder.build()?.try_deserialize().map_err(|e| e.into())
    }
}