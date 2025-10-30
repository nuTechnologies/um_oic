# 🦀 UM-OIC: Dual-Service Authentication System

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://www.docker.com)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A minimalistic, production-ready OAuth2/OIDC authentication system built in Rust, designed for educational institutions and multi-tenant environments with organization-based user management.

## ✨ Features

### 🔐 Authentication & Authorization
- **OAuth2/OIDC Provider** with PKCE support
- **JWT-based stateless authentication**
- **Multi-factor authentication** (TOTP)
- **Organization-scoped user management**
- **Claims registry** for centralized validation

### 🏢 Multi-Tenant Architecture
- **Organization-based data isolation**
- **Admin scoping** (global or per-organization)
- **Claims validation** per organization context
- **Scalable user directory structure**

### ⚡ Performance & Reliability
- **<10ms response times** with in-memory indices
- **Hot-reload capability** via SIGHUP signals
- **Atomic file operations** for data consistency
- **Memory-first storage** with O(1) lookups

### 🛠️ Operational Excellence
- **Service isolation** (auth + admin separation)
- **Container-ready** with Docker Compose
- **Structured logging** with JSON output
- **CLI tools** for operations and maintenance

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+
- Docker & Docker Compose (for containerized deployment)
- OpenSSL development libraries

### 1. Clone and Build

```bash
git clone <repository-url>
cd um_oic

# Setup development environment
make setup

# Build all services and install to bin/
make all

# Or build only Rust services
make build install
```

### 2. Initialize Data Directory

```bash
# Create initial admin user
./bin/auth-ops user create \
  --email admin@example.com \
  --password secure-password \
  --org all \
  --admin all
```

### 3. Configure Services

Create `config.toml`:

```toml
[server]
bind_auth = "0.0.0.0:8000"
bind_admin = "0.0.0.0:8001"

[security]
jwt_secret = "your-secret-key-here"
password_min_length = 12
access_token_ttl = 3600
refresh_token_ttl = 2592000

[instance]
name = "Your Organization"
issuer = "https://auth.example.com"
```

### 4. Start Services

**Development:**
```bash
# All services at once
make dev

# Or manually:
# Terminal 1: Auth Service
RUST_LOG=info ./bin/auth-service

# Terminal 2: Admin Service
RUST_LOG=info ./bin/admin-service

# Terminal 3: Admin App Dev Server
cd admin-app && npm run dev
```

**Production (Docker):**
```bash
docker-compose up -d
```

### 5. Access Services

- **Authentication**: http://localhost:8000
- **Administration**: http://localhost:8001
- **Health Checks**:
  - http://localhost:8000/health
  - http://localhost:8001/health

## 📁 Project Structure

```
um_oic/
├── auth-service/           # Authentication service (port 8000)
│   ├── src/
│   │   ├── main.rs         # Service entry point
│   │   ├── handlers/       # HTTP request handlers
│   │   ├── models.rs       # Data models
│   │   ├── storage_org.rs  # Organization-based storage
│   │   ├── jwt.rs          # JWT token handling
│   │   └── middleware/     # Security middleware
│   └── Dockerfile
├── admin-service/          # Administration service (port 8001)
│   ├── src/
│   │   ├── main.rs         # Service entry point
│   │   ├── handlers/       # Admin API handlers
│   │   ├── storage.rs      # Admin storage layer
│   │   └── middleware/     # Authentication middleware
│   └── Dockerfile
├── auth-ops/               # CLI operational tools
│   ├── src/
│   │   ├── main.rs         # CLI entry point
│   │   ├── backup.rs       # Backup utilities
│   │   └── storage.rs      # Data management
│   └── Dockerfile
├── admin-app/              # Vue.js Admin Application
│   ├── src/
│   │   ├── main.ts         # Vue app entry point
│   │   ├── components/     # Vue components
│   │   ├── views/          # Page views
│   │   ├── stores/         # Pinia stores
│   │   └── router/         # Vue Router
│   ├── package.json
│   └── vite.config.ts
├── bin/                    # Compiled binaries
│   ├── auth-service        # Auth service binary
│   ├── admin-service       # Admin service binary
│   └── auth-ops            # CLI tools binary
├── data/                   # Shared data directory
│   ├── web/                # Web assets
│   │   ├── auth/           # Login UI (static HTML)
│   │   └── mgmt/           # Admin UI (Vue.js build output)
│   ├── users/              # Organization-based users
│   │   ├── all/            # Global admin users
│   │   └── org-name/       # Organization users
│   ├── groups.json         # Group definitions
│   ├── clients.json        # OAuth2 clients
│   └── claims.conf         # Claims registry
├── Makefile               # Build automation
├── docker-compose.yml     # Container orchestration
├── Caddyfile             # Reverse proxy config
├── ARCHITECTURE.md       # Detailed architecture docs
└── README.md             # This file
```

## 🔧 Configuration

### Environment Variables

| Variable | Service | Default | Description |
|----------|---------|---------|-------------|
| `AUTH_DATA_DIR` | auth | `./data` | Data directory path |
| `AUTH_BIND` | auth | `0.0.0.0:8000` | Auth service bind address |
| `ADMIN_BIND` | admin | `0.0.0.0:8001` | Admin service bind address |
| `AUTH_DEBUG` | both | `false` | Enable debug logging |
| `RUST_LOG` | both | `info` | Log level |

### Data Directory Structure

```
data/
├── claims.conf              # Claims registry
├── groups.json             # Group definitions
├── clients.json            # OAuth2 clients
└── users/                  # Organization-based users
    ├── all/                # Global administrators
    │   └── user-admin.json
    ├── school-main/        # Organization users
    │   ├── user-123.json
    │   └── user-456.json
    └── group-8b/
        └── user-789.json
```

### Sample Claims Registry (`claims.conf`)

```json
{
  "roles": {
    "type": "array",
    "items": {"type": "string", "enum": ["master", "editor", "staff", "guardian"]},
    "description": "Application roles",
    "default_allowed": true,
    "required": true
  },
  "participant_ids": {
    "type": "array",
    "items": {"type": "string"},
    "description": "Associated participant IDs",
    "default_allowed": false,
    "sensitive": true
  }
}
```

## 🔑 API Documentation

### Authentication Service (Port 8000)

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/auth/login` | POST | User authentication |
| `/api/auth/logout` | POST | Session termination |
| `/oauth2/authorize` | GET | OAuth2 authorization |
| `/oauth2/token` | POST | Token exchange |
| `/oauth2/userinfo` | GET | User information |
| `/.well-known/openid-configuration` | GET | OIDC discovery |

### Administration Service (Port 8001)

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/users` | GET/POST | User management |
| `/api/users/:id` | GET/PATCH/DELETE | Individual user ops |
| `/api/organizations` | GET | Organization listing |
| `/api/organizations/:org/users` | GET | Org user listing |
| `/api/groups` | GET/POST | Group management |
| `/api/clients` | GET/POST | OAuth2 client management |

### CLI Tools

```bash
# User management
./auth-ops user create --email user@org.com --org school-main
./auth-ops user list --org school-main
./auth-ops user update --id user-123 --status inactive

# Data operations
./auth-ops backup create --output backup.tar.gz
./auth-ops backup restore --input backup.tar.gz
./auth-ops validate --data-dir ./data

# OAuth2 clients
./auth-ops client create --name "Web App" --redirect-uri https://app.com/callback
```

## 🐳 Docker Deployment

### Basic Deployment

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f auth admin

# Scale admin service
docker-compose up -d --scale admin=2
```

### With Reverse Proxy

```bash
# Start with Caddy proxy
docker-compose --profile with-proxy up -d
```

### Production Configuration

```yaml
# docker-compose.prod.yml
version: '3.8'
services:
  auth:
    image: um-oic/auth-service:latest
    environment:
      RUST_LOG: warn
      AUTH_DEBUG: "false"
    volumes:
      - /opt/um-oic/data:/data:ro
      - /opt/um-oic/certs:/certs:ro
    restart: always

  admin:
    image: um-oic/admin-service:latest
    environment:
      RUST_LOG: warn
      ADMIN_DEBUG: "false"
    volumes:
      - /opt/um-oic/data:/data
      - /opt/um-oic/certs:/certs:ro
    restart: always
```

## 🔒 Security Considerations

### Authentication Security
- **Argon2id password hashing** with secure parameters
- **JWT tokens** with configurable expiration
- **MFA support** via TOTP
- **Rate limiting** via reverse proxy
- **PKCE** for OAuth2 flows

### Multi-Tenant Security
- **Organization data isolation** via directory structure
- **Admin scope validation** for cross-org operations
- **Claims registry** prevents privilege escalation
- **Sensitive data marking** in claims definitions

### Operational Security
- **Service isolation** (auth read-only, admin read-write)
- **Atomic file operations** prevent corruption
- **Structured audit logging** for compliance
- **Health checks** for monitoring

## 📊 Performance Tuning

### Memory Optimization
```bash
# Reduce memory usage
export RUST_MIN_STACK=2097152

# Optimize for small datasets
export AUTH_CACHE_SIZE=1000
```

### Response Time Optimization
- Data is loaded into memory at startup
- O(1) lookups via HashMap indices
- Async I/O for file operations
- Static asset serving with zero-copy

### Scaling Considerations
- **Horizontal scaling**: Multiple auth-service instances
- **Data sharing**: Shared NFS/EFS for data directory
- **Load balancing**: Round-robin for auth endpoints
- **Caching**: Redis layer for high-volume deployments

## 🛠️ Development

### Prerequisites
```bash
rustup install 1.75
cargo install cargo-watch
cargo install cargo-audit
```

### Development Workflow
```bash
# Watch and rebuild
cargo watch -x "run -p auth-service"

# Run tests
cargo test
cargo test --package auth-service

# Lint and format
cargo clippy
cargo fmt

# Security audit
cargo audit
```

### Testing
```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Load testing (requires wrk)
wrk -t12 -c400 -d30s http://localhost:8000/health
```

## 📋 Operational Tasks

### Data Management
```bash
# Backup data
./auth-ops backup create --output "backup-$(date +%Y%m%d).tar.gz"

# Restore data
./auth-ops backup restore --input backup-20251030.tar.gz

# Validate data integrity
./auth-ops validate --data-dir ./data
```

### Service Management
```bash
# Reload auth-service data
kill -HUP $(cat /var/run/auth-service.pid)

# Check service health
curl http://localhost:8000/health
curl http://localhost:8001/health

# View service logs
journalctl -u auth-service -f
journalctl -u admin-service -f
```

### User Management
```bash
# Create organization admin
./auth-ops user create \
  --email admin@school.edu \
  --org school-main \
  --admin school-main

# Create regular user
./auth-ops user create \
  --email teacher@school.edu \
  --org school-main \
  --claims '{"roles":["staff"]}'

# Reset user password
./auth-ops user reset-password --email user@school.edu
```

## 🔍 Monitoring & Debugging

### Health Endpoints
- `GET /health` - Service health status
- `GET /api/system/status` - Detailed system status (admin only)

### Structured Logging
```json
{
  "timestamp": "2025-10-30T12:00:00Z",
  "level": "INFO",
  "service": "auth-service",
  "event": "user_login",
  "user_id": "user-550e8400",
  "org": "school-main",
  "duration_ms": 15
}
```

### Debug Mode
```bash
# Enable debug logging
export RUST_LOG=debug
export AUTH_DEBUG=true

# Trace requests
export RUST_LOG=auth_service=trace
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Run `cargo test` and `cargo fmt`
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Code Style
- Use `cargo fmt` for formatting
- Follow Rust naming conventions
- Add documentation for public APIs
- Include tests for new features

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum) web framework
- Authentication powered by [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- Password hashing via [Argon2](https://github.com/RustCrypto/password-hashes)
- Container orchestration with [Docker Compose](https://docs.docker.com/compose/)

## 📞 Support

- **Documentation**: See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architecture
- **Issues**: Open an issue on the repository
- **Discussions**: Use GitHub Discussions for questions

---

**Production-ready, minimal, secure.** 🦀✨