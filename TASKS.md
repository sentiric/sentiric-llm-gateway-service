# 🧠 LLM Gateway Service - Görev Listesi

Bu liste, bu repoyu devralacak geliştirici için öncelikli işleri sıralar.

## 🔴 Faz 1: İskelet ve Bağlantı
- [ ] **Protobuf Entegrasyonu:** `sentiric-contracts` reposunu ekle ve `build.rs` ile derle.
- [ ] **Llama Client:** `llm-llama-service` (C++) ile konuşacak gRPC istemcisini yaz.

## 🟡 Faz 2: Routing ve Stream
- [ ] **Router Struct:** `model_selector` stringini parse eden (Provider/Model ayrıştıran) yapıyı kur.
- [ ] **Stream Forwarding:** Llama servisinden gelen `GenerateStreamResponse` akışını, değişiklik yapmadan (veya sadece wrapper ekleyerek) istemciye ilet.
- [ ] **Gemini Client:** (Opsiyonel) Bulut yedeği için Gemini istemcisini entegre et.

## 🟢 Faz 3: Gelişmiş Özellikler
- [ ] **Fallback Logic:** `tonic::Status` kodlarını kontrol et. `UNAVAILABLE` gelirse yedeği dene.
- [ ] **History Handling:** Gelen `history` mesajlarını doğru formatta alt servise ilet.

## 🔵 Faz 4: Güvenlik
- [ ] **mTLS:** Güvenli bağlantıyı aktif et.