# 🧠 Akış Mantığı

1.  **GenerateStream (gRPC):** İstemci, bir `prompt` ve `model_preference` (local/cloud) gönderir.
2.  **Karar Mekanizması:**
    *   Eğer `model_preference == "local"` -> `LLM_LLAMA_SERVICE_GRPC_URL`
    *   Eğer `model_preference == "cloud"` -> `LLM_GEMINI_SERVICE_URL`
3.  **Proxy:** Gateway, seçilen motora gRPC veya REST isteği açar.
4.  **Streaming:** Gelen token'ları anlık olarak istemciye iletir (gRPC Stream).