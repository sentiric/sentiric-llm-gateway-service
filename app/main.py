# sentiric-llm-gateway-service/app/main.py
from fastapi import FastAPI, Depends, HTTPException, status
from contextlib import asynccontextmanager
from app.core.logging import setup_logging
from app.core.config import settings
import structlog

logger = structlog.get_logger(__name__)

@asynccontextmanager
async def lifespan(app: FastAPI):
    setup_logging()
    logger.info("LLM Gateway Service başlatılıyor", version=settings.SERVICE_VERSION, env=settings.ENV)
    
    # TODO: gRPC istemcileri (Gemini, Ollama vb.) burada başlatılacak.
    
    yield
    
    logger.info("LLM Gateway Service kapatılıyor")

app = FastAPI(
    title="Sentiric LLM Gateway Service",
    description="Akıllı LLM Yönlendiricisi",
    version=settings.SERVICE_VERSION,
    lifespan=lifespan
)

# LLM Routing endpoint'leri burada import edilecek
# from .api.v1 import llm_routes
# app.include_router(llm_routes.router, prefix=settings.API_V1_STR, tags=["LLM Routing"])

@app.get("/health", status_code=status.HTTP_200_OK)
async def health_check():
    # Placeholder: Sadece sunucunun ayakta olduğunu kontrol eder.
    return {"status": "ok", "service": "llm-gateway"}