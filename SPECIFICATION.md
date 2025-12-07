# 📋 Teknik Şartname (Specification)

## 1. Servis Kimliği
*   **Adı:** `sentiric-llm-gateway-service`
*   **Dil:** Rust (Tokio / Tonic)
*   **Port Bloğu:** 1602X (Harmonik Mimari)

## 2. API Kontratı (gRPC)

Servis, `sentiric-contracts` reposundaki `sentiric.llm.v1` paketini implemente eder.

### Proto Tanımı (`llm/v1/gateway.proto`)

```protobuf
service LlmGatewayService {
  rpc GenerateStream(GenerateStreamRequest) returns (stream GenerateStreamResponse);
}

message GenerateStreamRequest {
  string system_prompt = 1;     // AI Kişiliği (örn: "Sen yardımsever bir asistansın")
  string user_prompt = 2;       // Kullanıcı girdisi
  string model_selector = 3;    // örn: "local", "gemini", "gpt-4"
  
  repeated ChatMessage history = 4; // Konuşma geçmişi (Context)
}

message GenerateStreamResponse {
  string token = 1;             // Üretilen metin parçası
  string engine_used = 2;       // Hangi motorun cevap verdiği
}
```

## 3. Ortam Değişkenleri

| Değişken | Zorunlu | Açıklama |
| :--- | :--- | :--- |
| `LLM_GATEWAY_SERVICE_GRPC_PORT` | Evet | 16021 |
| `LLM_LLAMA_SERVICE_GRPC_URL` | Evet | http://llm-llama-service:16071 |
| `LLM_GEMINI_SERVICE_GRPC_URL` | Hayır | http://llm-gemini-service:16031 |
| `ENABLE_FALLBACK` | Hayır | `true` ise yerel motor çökünce buluta gider. |

## 4. Performans Hedefleri

*   **Overhead:** Gateway'in eklediği gecikme < 2ms olmalıdır.
*   **Concurrency:** Rust'ın asenkron yapısı sayesinde tek pod ile binlerce eş zamanlı diyalog akışını yönetebilmelidir.