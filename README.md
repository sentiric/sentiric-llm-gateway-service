# 🧠 Sentiric LLM Gateway Service

Platformun **Büyük Dil Modeli (LLM)** giriş noktasıdır. `dialog-service`'ten gelen metin üretim isteklerini (Chat Completion) alır ve `llm-llama-service` (Yerel) veya bulut sağlayıcılara (Gemini, OpenAI) yönlendirir.

## 🚀 Özellikler
*   **Model Router:** İsteğe göre (model_id) yerel veya bulut motora yönlendirme.
*   **Streaming Proxy:** Token'ları üretildikleri anda istemciye iletir (Low Latency).
*   **Unified API:** Arka plandaki motor ne olursa olsun dışarıya tek bir `GenerateStream` RPC'si sunar.

## 📦 Kurulum
```bash
make setup
make up
```

## 🔌 API
*   **gRPC (16021):** `sentiric.llm.v1.LlmGatewayService`