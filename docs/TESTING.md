# UM-OIC Testing Guide

Complete testing documentation for the UM-OIC authentication system.

## 🚀 Quick Start

### Option 1: Complete Test Suite (Recommended)
```bash
chmod +x run-tests.sh
./run-tests.sh
```
**What it does:**
- Builds all components
- Runs unit tests
- Tests CLI tool
- Starts services (auth + admin)
- Runs API integration tests
- Tests TLS certificates
- Performs basic performance tests

### Option 2: Local Development Setup
```bash
chmod +x start-local.sh
./start-local.sh
```
**What it does:**
- Starts services for development
- Auth service: https://localhost:8443 (TLS)
- Admin service: http://localhost:8001 (HTTP)
- Keeps services running for manual testing

### Option 3: Docker Testing (if Docker available)
```bash
chmod +x test-docker.sh
./test-docker.sh
```
**What it does:**
- Builds Docker images
- Tests docker-compose.tls.yml
- Verifies TLS in containerized environment

## 🔧 Manual Testing Options

### Individual Component Tests

#### 1. Unit Tests Only
```bash
cargo test
```

#### 2. CLI Tool Testing
```bash
cd auth-ops
cargo run -- --help
cargo run -- --data-dir ../data status
cargo run -- --data-dir ../data verify
```

#### 3. API Testing Scripts
```bash
chmod +x test-api.sh
./test-api.sh
```

#### 4. TLS Testing Scripts
```bash
chmod +x test-tls.sh
./test-tls.sh
```

### Manual Service Startup

#### Start Auth Service (HTTPS)
```bash
# With TLS (self-signed)
AUTH_TLS_ENABLE=true \
TLS_AUTO_GENERATE=true \
DOMAIN=localhost \
TLS_CERT_PATH=./certs/cert.pem \
TLS_KEY_PATH=./certs/key.pem \
AUTH_TLS_BIND=0.0.0.0:8443 \
AUTH_PID_FILE=./auth-service.pid \
./target/debug/auth-service \
    --tls-enable \
    --data-dir ./data \
    --config ./auth-service/config.toml
```

#### Start Admin Service (HTTP)
```bash
cd admin-service
ADMIN_BIND=0.0.0.0:8001 \
ADMIN_PID_FILE=../admin-service.pid \
AUTH_SERVICE_URL=https://localhost:8443 \
cargo run -- \
    --data-dir ../data \
    --config config.toml
```

## 🌐 Test Endpoints

### Auth Service (HTTPS)
- **Health Check**: `https://localhost:8443/health`
- **OIDC Discovery**: `https://localhost:8443/.well-known/openid-configuration`
- **OAuth2 Authorize**: `https://localhost:8443/oauth2/authorize`
- **OAuth2 Token**: `https://localhost:8443/oauth2/token`
- **User Info**: `https://localhost:8443/oauth2/userinfo`

### Admin Service (HTTP)
- **Health Check**: `http://localhost:8001/health` (requires auth)
- **Users API**: `http://localhost:8001/api/users` (requires auth)
- **Groups API**: `http://localhost:8001/api/groups` (requires auth)
- **Clients API**: `http://localhost:8001/api/clients` (requires auth)

## 🧪 Sample API Calls

### Test Health Endpoints
```bash
# Auth service health
curl -k https://localhost:8443/health | jq

# Admin service health (expects 401)
curl http://localhost:8001/health
```

### Test OIDC Discovery
```bash
curl -k https://localhost:8443/.well-known/openid-configuration | jq
```

### Test OAuth2 Authorization Flow
```bash
curl -k -G 'https://localhost:8443/oauth2/authorize' \
  --data-urlencode 'response_type=code' \
  --data-urlencode 'client_id=test-client' \
  --data-urlencode 'redirect_uri=https://example.com/callback' \
  --data-urlencode 'scope=openid profile email' | jq
```

### Test TLS Certificate
```bash
openssl s_client -connect localhost:8443 -servername localhost < /dev/null
```

## 👤 User Management via CLI

### Create Test User
```bash
cd auth-ops
cargo run -- --data-dir ../data user create \
  --email admin@test.local \
  --password password123 \
  --first-name Admin \
  --last-name User \
  --roles admin
```

### List Users
```bash
cd auth-ops
cargo run -- --data-dir ../data user list
```

### Reset Password
```bash
cd auth-ops
cargo run -- --data-dir ../data user reset-password admin@test.local newpassword
```

## 🐳 Docker Testing

### Prerequisites
- Docker and Docker Compose installed
- `docker-compose.tls.yml` file present

### Run Docker Tests
```bash
# Complete Docker test suite
./test-docker.sh

# Manual Docker startup
docker-compose -f docker-compose.tls.yml up -d

# Check container status
docker-compose -f docker-compose.tls.yml ps

# View logs
docker-compose -f docker-compose.tls.yml logs auth-service
docker-compose -f docker-compose.tls.yml logs admin-service

# Cleanup
docker-compose -f docker-compose.tls.yml down -v
```

## 🔍 Performance Testing

### Basic Performance Test
```bash
# Test response times
for i in {1..10}; do
  time curl -k -s https://localhost:8443/health > /dev/null
done
```

### Load Testing (if available)
```bash
# Using Apache Bench (if installed)
ab -n 100 -c 10 -k https://localhost:8443/health

# Using curl in parallel
seq 1 100 | xargs -n1 -P10 -I{} curl -k -s https://localhost:8443/health
```

## 🐛 Debugging

### View Logs
```bash
# Service logs (if configured)
tail -f auth-service/logs/auth-service.log
tail -f admin-service/logs/admin-service.log

# Real-time logs during development
RUST_LOG=debug ./target/debug/auth-service --tls-enable ...
```

### Check Process Status
```bash
# Check if services are running
ps aux | grep auth-service
ps aux | grep admin-service

# Check port usage
netstat -tlnp | grep -E "(8443|8001)"
```

### TLS Debugging
```bash
# Check certificate details
openssl x509 -in certs/cert.pem -text -noout

# Test TLS connection
openssl s_client -connect localhost:8443 -servername localhost

# Check cipher suites
nmap --script ssl-enum-ciphers -p 8443 localhost
```

## 📊 Test Coverage

### Automated Tests Cover:
- ✅ Unit tests for all modules
- ✅ CLI tool functionality
- ✅ API endpoint responses
- ✅ TLS certificate generation
- ✅ OAuth2/OIDC flows
- ✅ Security headers
- ✅ HTTP/2 support
- ✅ Service health checks
- ✅ Data validation
- ✅ Backup/restore operations

### Manual Testing Needed:
- 🔍 User interface testing
- 🔍 Cross-browser compatibility
- 🔍 Load testing under stress
- 🔍 Security penetration testing
- 🔍 Production deployment scenarios

## 🚨 Troubleshooting

### Common Issues

#### "Permission denied" errors
```bash
# Fix PID file permissions
sudo chown $USER:$USER /tmp/auth-service.pid
# Or use custom PID file location
AUTH_PID_FILE=./auth-service.pid ./target/debug/auth-service ...
```

#### Port already in use
```bash
# Find process using port
lsof -i :8443
# Kill process
kill <PID>
```

#### TLS certificate issues
```bash
# Remove old certificates
rm -rf certs/
# Let services regenerate them
mkdir certs
```

#### Docker issues
```bash
# Reset Docker state
docker-compose -f docker-compose.tls.yml down -v
docker system prune -f
```

## 📝 Test Reports

### Generate Test Report
```bash
# Run all tests and save output
./run-tests.sh 2>&1 | tee test-report.txt

# Generate coverage report (if configured)
cargo tarpaulin --out Html
```

### CI/CD Integration
```yaml
# Example GitHub Actions workflow
name: UM-OIC Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Rust
        uses: actions-rs/toolchain@v1
      - name: Run tests
        run: ./run-tests.sh
```