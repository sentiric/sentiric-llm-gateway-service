# sentiric-llm-gateway-service/app/core/config.py
from pydantic_settings import BaseSettings, SettingsConfigDict
from typing import Optional

class Settings(BaseSettings):
    PROJECT_NAME: str = "Sentiric LLM Gateway Service"
    API_V1_STR: str = "/api/v1"
    
    ENV: str = "production"
    LOG_LEVEL: str = "INFO"
    SERVICE_VERSION: str = "0.1.0"
    
    # AI Engine URL'leri (gRPC veya HTTP olabilir)
    LLM_GEMINI_SERVICE_TARGET_HTTP_URL: str
    LLM_OLLAMA_SERVICE_TARGET_HTTP_URL: Optional[str] = None
    LLM_STREAMING_SERVICE_TARGET_HTTP_URL: Optional[str] = None
    
    # Redis (Caching için)
    REDIS_URL: str

    model_config = SettingsConfigDict(
        env_file=".env", 
        env_file_encoding='utf-8',
        extra='ignore'
    )

settings = Settings()