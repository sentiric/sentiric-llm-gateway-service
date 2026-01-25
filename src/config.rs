use config::{Config, File, Environment};
use serde::Deserialize;
use anyhow::Result;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    #[allow(dead_code)]
    pub env: String,
    pub rust_log: String,
    pub service_version: String,
    
    // Ağ Ayarları
    pub host: String,
    pub grpc_port: u16,
    pub http_port: u16, // EKLENDİ
    
    // Hedef Servis
    pub llm_llama_service_grpc_url: String, 

    // Güvenlik (mTLS)
    pub grpc_tls_ca_path: String,
    pub llm_gateway_service_cert_path: String,
    pub llm_gateway_service_key_path: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let builder = Config::builder()
            .add_source(File::with_name(".env").required(false))
            .add_source(Environment::default().separator("__"))
            
            // MANUAL OVERRIDES
            .set_override_option("host", env::var("LLM_GATEWAY_SERVICE_LISTEN_ADDRESS").ok())?
            .set_override_option("grpc_port", env::var("LLM_GATEWAY_SERVICE_GRPC_PORT").ok())?
            .set_override_option("http_port", env::var("LLM_GATEWAY_SERVICE_HTTP_PORT").ok())? // EKLENDİ
            .set_override_option("llm_llama_service_grpc_url", env::var("LLM_LLAMA_SERVICE_GRPC_URL").ok())?

            // DEFAULTS
            .set_default("env", "production")?
            .set_default("rust_log", "info,sentiric_llm_gateway=debug")?
            .set_default("service_version", "1.1.2")?
            
            .set_default("host", "0.0.0.0")?
            .set_default("grpc_port", 16021)?
            .set_default("http_port", 16020)? // EKLENDİ
            
            .set_default("llm_llama_service_grpc_url", "https://llm-llama-service:16071")?
            
            .set_default("grpc_tls_ca_path", "/sentiric-certificates/certs/ca.crt")?
            .set_default("llm_gateway_service_cert_path", "/sentiric-certificates/certs/llm-gateway-service.crt")?
            .set_default("llm_gateway_service_key_path", "/sentiric-certificates/certs/llm-gateway-service.key")?;

        builder.build()?.try_deserialize().map_err(|e| e.into())
    }
}