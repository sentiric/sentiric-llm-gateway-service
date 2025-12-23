.PHONY: help setup up down logs test build clean network

help: ## Komutları listeler
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'



setup: ## Ortam dosyasını hazırlar
	@if [ ! -f .env ]; then cp .env.example .env; echo "✅ .env oluşturuldu."; fi

# DÜZELTME: Sadece docker-compose.yml kullanılıyor
up: setup ## Servisleri başlatır (Force Rebuild)
	docker compose up --build -d

down: ## Servisleri durdurur
	docker compose down --remove-orphans

logs: ## Logları canlı izler
	docker compose logs -f llm-gateway-service

clean: ## Cache ve artifactleri temizler
	cargo clean
	rm -rf target/