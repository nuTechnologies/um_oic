#!/bin/bash
# UM-OIC Local Development Setup
# Startet alle Services für lokale Entwicklung

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() {
    echo -e "${BLUE}[DEV-SETUP]${NC} $1"
}

success() {
    echo -e "${GREEN}✅${NC} $1"
}

error() {
    echo -e "${RED}❌${NC} $1"
}

warn() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

# Configuration
AUTH_PORT=8443
ADMIN_PORT=8445
DATA_DIR="./data"
PID_FILE="./dev-pids.txt"

cleanup() {
    log "Stopping all services..."

    if [[ -f "$PID_FILE" ]]; then
        while read -r pid; do
            if [[ -n "$pid" && "$pid" =~ ^[0-9]+$ ]]; then
                kill "$pid" 2>/dev/null || true
                log "Stopped process $pid"
            fi
        done < "$PID_FILE"
        rm -f "$PID_FILE"
    fi

    # Cleanup PID files
    rm -f auth-service.pid admin-service.pid

    success "All services stopped"
}

# Trap cleanup on exit
trap cleanup EXIT

echo "🚀 UM-OIC Local Development Setup"
echo "=================================="
echo

# Check if already running
if [[ -f "$PID_FILE" ]]; then
    warn "Services may already be running. Cleaning up first..."
    cleanup
fi

# Build in debug mode for faster development
log "Building services (debug mode)..."
cargo build
if [[ $? -ne 0 ]]; then
    error "Build failed"
    exit 1
fi
success "Build completed"

# Setup data directory
mkdir -p "$DATA_DIR"
mkdir -p certs


# Start auth-service with TLS
log "Starting auth-service on https://localhost:$AUTH_PORT..."
AUTH_TLS_ENABLE=true \
TLS_AUTO_GENERATE=true \
DOMAIN=localhost \
TLS_CERT_PATH="./certs/dev-cert.pem" \
TLS_KEY_PATH="./certs/dev-key.pem" \
AUTH_TLS_BIND="0.0.0.0:$AUTH_PORT" \
AUTH_PID_FILE="./auth-service.pid" \
RUST_LOG=debug \
./target/debug/auth-service \
    --tls-enable \
    --data-dir "$DATA_DIR" \
    --config ./auth-service/config.toml \
    --debug &

AUTH_PID=$!
echo "$AUTH_PID" > "$PID_FILE"
log "Auth service PID: $AUTH_PID"

# Wait for auth service
log "Waiting for auth service to start..."
for i in {1..10}; do
    if curl -k -s "https://localhost:$AUTH_PORT/health" >/dev/null 2>&1; then
        success "Auth service is ready"
        break
    fi
    if [[ $i -eq 10 ]]; then
        error "Auth service failed to start"
        exit 1
    fi
    sleep 1
done

# Start admin-service (HTTPS)
log "Starting admin-service on https://localhost:$ADMIN_PORT..."
cd admin-service
ADMIN_TLS_ENABLE=true \
TLS_AUTO_GENERATE=true \
DOMAIN=admin.localhost \
TLS_CERT_PATH="../certs/admin-cert.pem" \
TLS_KEY_PATH="../certs/admin-key.pem" \
ADMIN_BIND="0.0.0.0:$ADMIN_PORT" \
ADMIN_PID_FILE="../admin-service.pid" \
AUTH_SERVICE_URL="https://localhost:$AUTH_PORT" \
RUST_LOG=debug \
cargo run -- \
    --tls-enable \
    --data-dir "../$DATA_DIR" \
    --config config.toml \
    --debug &

ADMIN_PID=$!
echo "$ADMIN_PID" >> "../$PID_FILE"
cd ..
log "Admin service PID: $ADMIN_PID"

# Wait for admin service
log "Waiting for admin service to start..."
for i in {1..10}; do
    HTTP_STATUS=$(curl -k -s -o /dev/null -w "%{http_code}" "https://localhost:$ADMIN_PORT/health" 2>/dev/null || echo "000")
    if [[ $HTTP_STATUS -eq 200 ]]; then
        success "Admin service is ready"
        break
    fi
    if [[ $i -eq 10 ]]; then
        warn "Admin service may not be fully ready (status: $HTTP_STATUS)"
        break
    fi
    sleep 1
done

# Show service status
echo
log "=== Service Status ==="
echo "🔒 Auth Service:  https://localhost:$AUTH_PORT (TLS enabled)"
echo "🔧 Admin Service: https://localhost:$ADMIN_PORT (HTTPS)"
echo "📁 Data Directory: $DATA_DIR"
echo

# Test endpoints
log "=== Quick Tests ==="
AUTH_HEALTH=$(curl -k -s "https://localhost:$AUTH_PORT/health" 2>/dev/null || echo '{"status":"error"}')
AUTH_STATUS=$(echo "$AUTH_HEALTH" | jq -r .status 2>/dev/null || echo "error")

ADMIN_STATUS=$(curl -k -s -o /dev/null -w "%{http_code}" "https://localhost:$ADMIN_PORT/health" 2>/dev/null || echo "000")

if [[ "$AUTH_STATUS" == "healthy" ]]; then
    success "Auth service: $AUTH_STATUS"
else
    error "Auth service: $AUTH_STATUS"
fi

if [[ "$ADMIN_STATUS" == "401" ]]; then
    success "Admin service: Protected (401)"
elif [[ "$ADMIN_STATUS" == "200" ]]; then
    success "Admin service: Available (200)"
else
    warn "Admin service: Status $ADMIN_STATUS"
fi

# CLI tool test
log "=== CLI Tool Available ==="
echo "cd auth-ops && cargo run -- --data-dir ../$DATA_DIR status"

# Development URLs
echo
log "=== Development URLs ==="
echo "📊 Health Check:      https://localhost:$AUTH_PORT/health"
echo "🔐 OIDC Discovery:    https://localhost:$AUTH_PORT/.well-known/openid-configuration"
echo "🔑 OAuth2 Authorize:  https://localhost:$AUTH_PORT/oauth2/authorize"
echo "👤 Admin Health:      https://localhost:$ADMIN_PORT/health"
echo "👥 Admin Users:       https://localhost:$ADMIN_PORT/api/users"

# Sample commands
echo
log "=== Sample Commands ==="
echo "# Test auth service"
echo "curl -k https://localhost:$AUTH_PORT/health | jq"
echo
echo "# Test OAuth2 flow"
echo "curl -k -G 'https://localhost:$AUTH_PORT/oauth2/authorize' \\"
echo "  --data-urlencode 'response_type=code' \\"
echo "  --data-urlencode 'client_id=test-client' \\"
echo "  --data-urlencode 'redirect_uri=https://example.com/callback'"
echo
echo "# Create user via CLI"
echo "cd auth-ops && cargo run -- --data-dir ../$DATA_DIR user create \\"
echo "  --email admin@dev.local --password dev123 \\"
echo "  --first-name Dev --last-name Admin --roles admin"
echo
echo "# View logs"
echo "tail -f auth-service/logs/auth-service.log"

# Keep services running
echo
log "Services are running in development mode"
log "Press Ctrl+C to stop all services"
echo

# Wait for interrupt
wait
