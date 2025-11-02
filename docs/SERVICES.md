# UM-OIC Services Overview

## Architecture
- **auth-service**: OAuth2/OIDC provider (Port 8443 HTTPS)
- **admin-service**: Web-based admin interface (Port 8444 HTTPS)
- **auth-ops**: CLI tool for user management

## Quick Start
```bash
./start-local.sh
```

## Ports
- Auth: https://localhost:8443 (HTTPS)
- Admin UI: https://localhost:8445 (HTTPS)
- Health: /health on both

## Data Structure
- Claims: `data/claims.json` (renamed from .conf)
- Users: `data/users/{org}/*.json` (org-based directories)


## Auth Flow
Admin UI redirects to auth service for login, returns with token.

