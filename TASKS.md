# 🧠 Sentiric LLM Gateway Service - Görev Listesi

Bu servisin mevcut ve gelecekteki tüm geliştirme görevleri, platformun merkezi görev yönetimi reposu olan **`sentiric-tasks`**'ta yönetilmektedir.

➡️ **[Aktif Görev Panosuna Git](https://github.com/sentiric/sentiric-tasks/blob/main/TASKS.md)**

---
Bu belge, servise özel, çok küçük ve acil görevler için geçici bir not defteri olarak kullanılabilir.

## Faz 1: Minimal İşlevsellik (INFRA-02)
- [x] Temel Python projesi, Poetry ve Dockerfile oluşturuldu.
- [x] FastAPI ve Uvicorn iskeleti kuruldu, Health Check hazırlandı.
- [ ] Gelen LLM gRPC isteklerini alıp HTTP'ye çevirecek bir ara katman (middleware) eklenecek. (AI-LLM-01)
- [ ] En az iki uzmana (Gemini Mock, Ollama Mock) yönlendirme (routing) mantığı implemente edilecek. (AI-LLM-02)