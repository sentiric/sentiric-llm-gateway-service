# 🧠 Sentiric LLM Gateway Service

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![Security](https://img.shields.io/badge/security-mTLS-green.svg)]()
[![Architecture](https://img.shields.io/badge/architecture-layer_3_gateway-blue.svg)]()

**Sentiric İletişim İşletim Sistemi**'nin "Zeka Dağıtım Merkezi"dir. Platformdaki tüm Büyük Dil Modeli (LLM) isteklerini karşılar, güvenli bir tünel (mTLS) üzerinden uygun motora (Yerel Llama vb.) yönlendirir ve yanıtı anlık olarak (Streaming) istemciye iletir.

## 🎯 Temel Yetenekler

1.  **Güvenli Yönlendirme (Secure Routing):** İstemci ve Uzman Motorlar arasındaki trafiği **mTLS (Karşılıklı TLS)** ile şifreler.
2.  **Akıllı Seçim:** `model_selector` parametresine göre trafiği Yerel (Llama) veya Bulut motorlarına yönlendirir.
3.  **Protokol Dönüşümü:** `GenerateDialogStream` (Gateway) formatını `GenerateStream` (Motor) formatına dönüştürür.
4.  **Yüksek Performans:** Rust (Tokio/Tonic) tabanlı mimarisi ile <2ms gecikme (overhead) ekler.

## 🏗️ Mimari Konum

*   **Üst Akış (Callers):** `sentiric-dialog-service`
*   **Alt Akış (Upstreams):** `sentiric-llm-llama-service` (gRPC/mTLS)

## 📦 Kurulum ve Çalıştırma

### Gereksinimler
*   Rust (1.75+)
*   `sentiric-certificates` tarafından üretilmiş sertifikalar (`/certs` dizininde olmalı).

### Ortam Değişkenleri (.env)
```bash
# Servis Ayarları
HOST=0.0.0.0
GRPC_PORT=16021

# Hedef Motor
LLM_LLAMA_URL=http://llm-llama-service:16071

# Güvenlik (Zorunlu)
GRPC_TLS_CA_PATH=../sentiric-certificates/certs/ca.crt
LLM_GATEWAY_SERVICE_CERT_PATH=../sentiric-certificates/certs/llm-gateway-service.crt
LLM_GATEWAY_SERVICE_KEY_PATH=../sentiric-certificates/certs/llm-gateway-service.key
```

### Başlatma
```bash
# Local Development
make up

# Production Build
cargo build --release
```