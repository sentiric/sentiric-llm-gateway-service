# 🚀 Sentiric LLM Gateway - Kullanım ve Test Rehberi

Bu belge, **Sentiric LLM Gateway** servisini başlatmak, test etmek ve sorun gidermek için gerekli komutları içerir.

## 1. Hızlı Başlangıç (Quick Start)

Sistemi sıfırdan ayağa kaldırmak için:

```bash
# 1. Eski konteynerleri ve ağları temizle
make down

# 2. Servisleri başlat (Sertifikaları ve Modelleri otomatik hazırlar)
make up

# 3. Logları izle (Hazır olana kadar bekle)
make logs
```

**Hazır Olma Belirtisi:**
Loglarda `🚀 gRPC server listening on 0.0.0.0:16021` mesajını gördüğünüzde sistem hazırdır.

---

## 2. Test Etme (Verification)

Sistem mTLS (Karşılıklı TLS) ile korunduğu için, test istekleri sertifikalarla birlikte gönderilmelidir.

### 2.1. Basit "Merhaba" Testi (Linux/Mac)

Aşağıdaki komut, Docker üzerinden `grpcurl` çalıştırarak Gateway'e güvenli bir istek atar:

```bash
docker run --rm --network sentiric-llm-gateway-service_sentiric-net \
  -v $(pwd)/../sentiric-contracts/proto:/proto \
  -v $(pwd)/../sentiric-certificates/certs:/certs \
  fullstorydev/grpcurl \
  -import-path /proto \
  -proto sentiric/llm/v1/gateway.proto \
  -cacert /certs/ca.crt \
  -cert /certs/llm-gateway-service.crt \
  -key /certs/llm-gateway-service.key \
  -H 'x-trace-id: test-12345' \
  -d '{"model_selector": "local", "llama_request": {"user_prompt": "Merhaba"}}' \
  llm-gateway-service:16021 \
  sentiric.llm.v1.LlmGatewayService/GenerateDialogStream \
  | grep 'token":' | awk -F'"' '{print $4}' | base64 -d
```

**Beklenen Çıktı:**
Base64 formatında parça parça gelen JSON yanıtları:
```json
{ "llamaResponse": { "token": "TWVyaGFiYQ==" } }
...
```

### 2.2. Base64 Decode (Okunabilir Çıktı)

Çıktıyı anlık olarak okumak için (Linux):

```bash
# Yukarıdaki komutun sonuna şunu ekleyin:
| grep 'token":' | awk -F'"' '{print $4}' | base64 -d
```

---

## 3. Sorun Giderme (Troubleshooting)

### Hata: `transport error`
*   **Sebep:** Gateway ve Llama servisi arasında TLS el sıkışması başarısız oldu.
*   **Çözüm:** Sertifikaların `sentiric.cloud` domainini içerdiğinden emin olun (`make logs` ile SSL hatasına bakın). Gerekirse sertifikaları yeniden üretin.

### Hata: `service not found` (Mock Modunda)
*   **Sebep:** Mock servisine (GripMock) cevap kuralı (Stub) öğretilmemiş.
*   **Çözüm:** `curl` ile stub ekleyin (README.md'ye bakın).

### Hata: `context deadline exceeded`
*   **Sebep:** Llama motoru modeli yüklüyor olabilir (Soğuk Başlangıç).
*   **Çözüm:** İlk istekte model RAM'e yüklendiği için 10-20 saniye beklemek normaldir. İkinci istekte hızlanacaktır.

---

## 4. Mimari Notlar

*   **Port:** `16021` (gRPC), `16020` (HTTP Metrics)
*   **Güvenlik:** mTLS (Zero Trust)
*   **Motor:** C++ Llama Engine (Gemma 3 Optimized)
