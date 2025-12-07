# 🧠 Sentiric LLM Gateway Service

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![Architecture](https://img.shields.io/badge/architecture-layer_3_gateway-blue.svg)]()
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)]()

**Sentiric İletişim İşletim Sistemi**'nin "Zeka Dağıtım Merkezi"dir. Platformdaki tüm Büyük Dil Modeli (LLM) isteklerini karşılar ve bunları maliyet, hız veya yetenek gereksinimlerine göre uygun motora (Yerel Llama, Bulut Gemini vb.) yönlendirir.

## 🎯 Temel Sorumluluklar

1.  **Model Yönlendirme (Model Routing):** İstek içindeki `model_selector` etiketine göre trafiği yönlendirir (örn: `local:gemma` -> Llama Service, `cloud:gemini` -> Gemini Service).
2.  **Token Streaming:** Zeka motorlarından gelen yanıtları (token) biriktirmeden, kelime kelime istemciye iletir. Bu, kullanıcının bekleme süresini (Latency) hissetmemesini sağlar.
3.  **Protokol Standardizasyonu:** Arka plandaki motorların farklı API'lerini (REST, gRPC) dış dünyaya tek bir standart gRPC arayüzü olarak sunar.
4.  **Yedekleme (Fallback):** Birincil motor (örn: Yerel GPU) yanıt vermezse, trafiği otomatik olarak ikincil motora (örn: Bulut) kaydırabilir.

## 🏗️ Mimari Konum

Bu servis **Katman 3 (Ağ Geçitleri)** seviyesinde yer alır.

*   **Üst Akış (Callers):** `dialog-service`, `agent-service`.
*   **Alt Akış (Downstreams):**
    *   `llm-llama-service` (Yerel / C++ / gRPC)
    *   `llm-gemini-service` (Bulut / Python / gRPC)

## 📦 Kurulum ve Çalıştırma

### Gereksinimler
*   Rust (1.75+)
*   Protobuf Compiler (`protoc`)

### Komutlar
```bash
# Ortamı hazırla
make setup

# Servisi başlat
make up

# Logları izle
make logs
```

## 🔌 API ve Portlar

*   **gRPC (16021):** `sentiric.llm.v1.LlmGatewayService`
*   **HTTP (16020):** `/health`, `/metrics`