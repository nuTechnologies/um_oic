"""
UM-OIC Manager Service Configuration
"""
import os
from pathlib import Path
from typing import Optional

from pydantic import BaseSettings


class Settings(BaseSettings):
    """Application settings"""

    # Service config
    service_name: str = "um-oic-manager"
    version: str = "0.1.0"
    debug: bool = False

    # Server config
    host: str = "0.0.0.0"
    port: int = 8446

    # TLS config
    tls_enabled: bool = True
    tls_cert_path: str = "./certs/manager-cert.pem"
    tls_key_path: str = "./certs/manager-key.pem"
    tls_auto_generate: bool = True
    domain: str = "manager.localhost"

    # Auth service integration
    auth_service_url: str = "https://localhost:8443"
    jwt_public_key_path: Optional[str] = None

    # Data directory
    data_dir: str = "./data"

    # Logging
    log_level: str = "INFO"

    class Config:
        env_prefix = "MANAGER_"
        env_file = ".env"


settings = Settings()