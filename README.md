### 📄 File: `README.md` | 🏷️ Markdown

```markdown
# 🧠 Sentiric LLM Gateway Service

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![Language](https://img.shields.io/badge/language-Python-blue.svg)]()
[![Framework](https://img.shields.io/badge/framework-FastAPI-blueviolet.svg)]()

**Sentiric LLM Gateway Service**, Sentiric platformunun LLM tüketimini merkezileştiren bir yönlendiricidir. Bu servis, diğer mikroservislerin tek bir LLM modeline (Gemini, Ollama, OpenAI) sabitlenmesini önler, maliyet optimizasyonu ve yüksek erişilebilirlik (HA) sağlar.

## 🎯 Temel Sorumluluklar

*   **Dinamik Model Yönlendirme:** Gelen isteğe göre en uygun LLM sağlayıcısını seçer.
*   **API Normaleştirme:** Harici LLM API'lerinin (REST/JSON) farklı arayüzlerini tek tip gRPC kontratına çevirir.
*   **Yük Dengeleme:** Uzman motorlar arasında trafiği dağıtır.
*   **Streaming Yönetimi:** Akış (streaming) isteklerini yönetir.

## 🛠️ Teknoloji Yığını

*   **Dil:** Python 3.11
*   **Web Çerçevesi:** FastAPI / Uvicorn
*   **Loglama:** Structlog (JSON formatında izlenebilir loglar için)
*   **Bağımlılıklar:** `sentiric-contracts` v1.9.0

## 🔌 API Etkileşimleri

*   **Gelen (Sunucu):**
    *   `sentiric-agent-service` (gRPC): `Generate`, `GenerateStream` RPC'leri.
*   **Giden (İstemci):**
    *   `sentiric-llm-gemini-service` (HTTP/gRPC)
    *   `sentiric-llm-ollama-service` (HTTP/gRPC)

---
## 🏛️ Anayasal Konum

Bu servis, [Sentiric Anayasası'nın](https://github.com/sentiric/sentiric-governance) **AI Gateway Layer**'ında yer alan merkezi bir bileşendir.