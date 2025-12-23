# 📋 Teknik Şartname

## 1. Servis Kimliği
*   **Adı:** `sentiric-llm-gateway-service`
*   **Dil:** Rust (Tokio / Tonic)
*   **Port Bloğu:** 1602X (Harmonik Mimari)

## 2. Kaynak Tüketimi
*   **CPU:** Idle durumda < %1, Yük altında (1000 req/s) < %10 (Tek Çekirdek)
*   **RAM:** < 50 MB (Stateless olduğu için)

## 3. API Kontratı
Servis, `sentiric-contracts` v1.12.3 sürümünü kullanır.
*   **Paket:** `sentiric.llm.v1`
*   **Servis:** `LlmGatewayService`
*   **Metod:** `GenerateDialogStream`

## 4. Hata Yönetimi
*   Upstream (Llama) ulaşılamazsa: `Status::UNAVAILABLE` döner.
*   Sertifika hatası varsa: Servis `panic` ile kapanır (Fail Fast).