use config::{Config, File, Environment};
use serde::Deserialize;
use anyhow::Result;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub env: String,
    pub rust_log: String,
    pub service_version: String,
    
    // Ağ Ayarları (Standardize Edilmiş)
    pub host: String,      // LLM_GATEWAY_SERVICE_LISTEN_ADDRESS
    pub grpc_port: u16,    // LLM_GATEWAY_SERVICE_GRPC_PORT
    
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
            // 1. Varsa .env dosyasını oku (Opsiyonel)
            .add_source(File::with_name(".env").required(false))
            
            // 2. Environment Variables (Otomatik Eşleşme)
            .add_source(Environment::default().separator("__"))

            // 3. MANUAL OVERRIDES (Senin Standardın - En Yüksek Öncelik)
            // Bu kısım, .env dosyasındaki veya docker env'deki karmaşık isimleri
            // bizim basit struct alanlarımıza map eder.
            
            // Host: LLM_GATEWAY_SERVICE_LISTEN_ADDRESS -> host
            .set_override_option("host", env::var("LLM_GATEWAY_SERVICE_LISTEN_ADDRESS").ok())?
            
            // Port: LLM_GATEWAY_SERVICE_GRPC_PORT -> grpc_port
            .set_override_option("grpc_port", env::var("LLM_GATEWAY_SERVICE_GRPC_PORT").ok())?
            
            // Hedef URL: LLM_LLAMA_SERVICE_GRPC_URL (Zaten standart)
            .set_override_option("llm_llama_service_grpc_url", env::var("LLM_LLAMA_SERVICE_GRPC_URL").ok())?

            // 4. SMART DEFAULTS (Hiçbir şey verilmezse çalışacak değerler)
            .set_default("env", "production")?
            .set_default("rust_log", "info,sentiric_llm_gateway=debug")?
            .set_default("service_version", "1.0.0")?
            
            // Standart Docker içi yollar
            .set_default("host", "0.0.0.0")?
            .set_default("grpc_port", 16021)?
            
            // Varsayılan hedef (Docker Service Name DNS)
            .set_default("llm_llama_service_grpc_url", "https://llm-llama-service:16071")?
            
            // Varsayılan Sertifika Yolları (Container İçi Standart Yol)
            .set_default("grpc_tls_ca_path", "/sentiric-certificates/certs/ca.crt")?
            .set_default("llm_gateway_service_cert_path", "/sentiric-certificates/certs/llm-gateway-service.crt")?
            .set_default("llm_gateway_service_key_path", "/sentiric-certificates/certs/llm-gateway-service.key")?;

        builder.build()?.try_deserialize().map_err(|e| e.into())
    }
}