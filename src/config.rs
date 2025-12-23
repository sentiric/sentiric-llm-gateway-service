use config::{Config, File, Environment};
use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub env: String,
    pub rust_log: String,
    pub service_version: String,
    
    pub host: String,
    pub grpc_port: u16,

    // DÜZELTME: Değişken adını netleştiriyoruz
    pub llm_llama_service_grpc_url: String, 

    pub grpc_tls_ca_path: String,
    pub llm_gateway_service_cert_path: String,
    pub llm_gateway_service_key_path: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let builder = Config::builder()
            .add_source(File::with_name(".env").required(false))
            // Environment değişkenlerini otomatik eşleştir (örn: LLM_LLAMA_URL -> llm_llama_url)
            .add_source(Environment::default().separator("__"))
            
            // Manuel Override (Docker Compose'daki uzun ismi desteklemek için)
            .set_override_option("llm_llama_service_grpc_url", std::env::var("LLM_LLAMA_SERVICE_GRPC_URL").ok())?
            
            // Varsayılan Değerler
            .set_default("env", "development")?
            .set_default("rust_log", "info")?
            .set_default("service_version", "1.0.0")?
            .set_default("host", "0.0.0.0")?
            .set_default("grpc_port", 16021)?
            // Eğer her şey başarısız olursa bu çalışır (DNS hatası veren bu)
            .set_default("llm_llama_url", "http://llm-llama-service:16071")?;

        builder.build()?.try_deserialize().map_err(|e| e.into())
    }
}