# Claude Code Development Notes for UM-OIC

## 🔐 CRITICAL: ALWAYS USE HTTPS FOR BOTH SERVICES

**USER REQUIREMENT (stated 147+ times):**
- **Auth-Service**: HTTPS on port 8443
- **Admin-Service**: HTTPS on port 8445 (NOT HTTP!)

**Never suggest HTTP for admin service again!**

## Default Service Configuration:
```bash
# Auth Service (HTTPS)
AUTH_TLS_ENABLE=true
AUTH_TLS_BIND=0.0.0.0:8443

# Admin Service (HTTPS) - ALWAYS HTTPS!
ADMIN_TLS_ENABLE=true
ADMIN_BIND=0.0.0.0:8445
```

## Quick Start Commands:
```bash
# Auth Service
cd auth-service
AUTH_TLS_ENABLE=true TLS_AUTO_GENERATE=true DOMAIN=localhost \
TLS_CERT_PATH=../certs/cert.pem TLS_KEY_PATH=../certs/key.pem \
AUTH_TLS_BIND=0.0.0.0:8443 cargo run -- --tls-enable --data-dir ../data --config config.toml

# Admin Service (HTTPS!)
cd admin-service
ADMIN_TLS_ENABLE=true TLS_AUTO_GENERATE=true DOMAIN=admin.localhost \
TLS_CERT_PATH=../certs/admin-cert.pem TLS_KEY_PATH=../certs/admin-key.pem \
ADMIN_BIND=0.0.0.0:8445 cargo run -- --tls-enable --data-dir ../data --config config.toml
```

## URLs:
- Auth: https://localhost:8443
- Admin: https://localhost:8445

## File Paths (corrected with symlinks):
- ln -sf ../data ./auth-service/data
- ln -sf ../data ./admin-service/data

## Docker Configuration:
Both services use HTTPS in containers too.

## Management Scripts (USE THESE!):
```bash
# Local Development
./start-local.sh         # Start both services
pkill -f "auth-service" || pkill -f "admin-service"  # Stop services

# Docker
./docker-run.sh up       # Start containers
./docker-run.sh down     # Stop containers

# Tests
./test/api/service-manager.sh start   # Start test services
./test/api/service-manager.sh stop    # Stop test services
```

**STOP manually starting services - USE THE SCRIPTS!**

---
**READ THIS FILE FIRST BEFORE ANY CONFIGURATION CHANGES!**