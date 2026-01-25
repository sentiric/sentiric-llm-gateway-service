// sentiric-llm-gateway-service/src/main.rs
mod app;
mod config;
mod error;
mod grpc;
mod clients;
mod tls;
mod metrics; // YENİ

use anyhow::Result;
use app::App;
use rustls::crypto::CryptoProvider;
use rustls::crypto::ring::default_provider;

#[tokio::main]
async fn main() -> Result<()> {
    // Kripto sağlayıcısını en başta kur.
    let provider = default_provider();
    CryptoProvider::install_default(provider).expect("Failed to install crypto provider");

    App::run().await
}