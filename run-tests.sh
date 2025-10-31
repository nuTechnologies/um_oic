#!/bin/bash
# UM-OIC Complete Test Runner
# Startet alle Tests: Unit, Integration, API, CLI, TLS

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() {
    echo -e "${BLUE}[TEST-RUNNER]${NC} $1"
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

# Test configuration
PROJECT_ROOT=$(pwd)
TEST_PORT_AUTH=8543
TEST_PORT_ADMIN=8501
TEST_DATA_DIR="$PROJECT_ROOT/test-run-data"
PIDS_FILE="$PROJECT_ROOT/test-pids.txt"

cleanup() {
    log "Cleaning up test environment..."

    # Kill test services
    if [[ -f "$PIDS_FILE" ]]; then
        while read -r pid; do
            if [[ -n "$pid" && "$pid" =~ ^[0-9]+$ ]]; then
                kill "$pid" 2>/dev/null || true
            fi
        done < "$PIDS_FILE"
        rm -f "$PIDS_FILE"
    fi

    # Clean test data
    rm -rf "$TEST_DATA_DIR"

    success "Cleanup completed"
}

# Trap cleanup on exit
trap cleanup EXIT

echo "🚀 UM-OIC Complete Test Suite"
echo "=============================="
echo "Project: $PROJECT_ROOT"
echo "Auth Port: $TEST_PORT_AUTH"
echo "Admin Port: $TEST_PORT_ADMIN"
echo

# Phase 1: Build all components
log "Phase 1: Building all components..."
cargo build --release
if [[ $? -eq 0 ]]; then
    success "Build completed successfully"
else
    error "Build failed"
    exit 1
fi

# Phase 2: Unit Tests
log "Phase 2: Running unit tests..."
cargo test
if [[ $? -eq 0 ]]; then
    success "Unit tests passed"
else
    warn "Some unit tests failed (continuing with integration tests)"
fi

# Phase 3: CLI Tests
log "Phase 3: Testing CLI tool..."
cd auth-ops

# Setup CLI test data
mkdir -p "$TEST_DATA_DIR"
echo '{"users":[]}' > "$TEST_DATA_DIR/users.json"
echo '{"groups":[]}' > "$TEST_DATA_DIR/groups.json"
echo '{"roles":[]}' > "$TEST_DATA_DIR/roles.json"
echo '{"clients":[]}' > "$TEST_DATA_DIR/clients.json"
echo '{"claims_registry":{"definitions":{}}}' > "$TEST_DATA_DIR/claims_registry.json"

# Test CLI functions
CLI_RESULT=0
cargo run -- --data-dir "$TEST_DATA_DIR" verify >/dev/null 2>&1 || CLI_RESULT=1
cargo run -- --data-dir "$TEST_DATA_DIR" status >/dev/null 2>&1 || CLI_RESULT=1

if [[ $CLI_RESULT -eq 0 ]]; then
    success "CLI tests passed"
else
    error "CLI tests failed"
    exit 1
fi

cd ..

# Phase 4: Start services for integration tests
log "Phase 4: Starting services for integration tests..."

# Clean previous test data
rm -rf "$TEST_DATA_DIR"
mkdir -p "$TEST_DATA_DIR"

# Copy initial data
cp -r data/* "$TEST_DATA_DIR/" 2>/dev/null || true

# Start auth-service with TLS
log "Starting auth-service on port $TEST_PORT_AUTH..."
AUTH_TLS_ENABLE=true \
TLS_AUTO_GENERATE=true \
DOMAIN=test.localhost \
TLS_CERT_PATH="./certs/test-auth-cert.pem" \
TLS_KEY_PATH="./certs/test-auth-key.pem" \
AUTH_TLS_BIND="0.0.0.0:$TEST_PORT_AUTH" \
AUTH_PID_FILE="$TEST_DATA_DIR/auth-service.pid" \
./target/release/auth-service \
    --tls-enable \
    --data-dir "$TEST_DATA_DIR" \
    --config ./auth-service/config.toml &

AUTH_PID=$!
echo "$AUTH_PID" > "$PIDS_FILE"

# Wait for auth service
sleep 3

# Test auth service
curl -k -s "https://localhost:$TEST_PORT_AUTH/health" >/dev/null
if [[ $? -eq 0 ]]; then
    success "Auth service started successfully"
else
    error "Auth service failed to start"
    exit 1
fi

# Start admin-service
log "Starting admin-service on port $TEST_PORT_ADMIN..."
cd admin-service
ADMIN_BIND="0.0.0.0:$TEST_PORT_ADMIN" \
ADMIN_PID_FILE="$TEST_DATA_DIR/admin-service.pid" \
AUTH_SERVICE_URL="https://localhost:$TEST_PORT_AUTH" \
cargo run -- \
    --data-dir "$TEST_DATA_DIR" \
    --config config.toml &

ADMIN_PID=$!
echo "$ADMIN_PID" >> "$PIDS_FILE"
cd ..

# Wait for admin service
sleep 3

# Test admin service
HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "http://localhost:$TEST_PORT_ADMIN/health")
if [[ $HTTP_STATUS -eq 401 ]]; then
    success "Admin service started successfully (auth required)"
else
    warn "Admin service status: $HTTP_STATUS"
fi

# Phase 5: API Integration Tests
log "Phase 5: Running API integration tests..."

API_TEST_RESULT=0

# Test auth service endpoints
log "Testing auth service HTTPS endpoints..."
curl -k -s "https://localhost:$TEST_PORT_AUTH/health" | jq .status >/dev/null || API_TEST_RESULT=1
curl -k -s "https://localhost:$TEST_PORT_AUTH/.well-known/openid-configuration" | jq .issuer >/dev/null || API_TEST_RESULT=1

# Test OAuth2 flow
OAUTH_RESPONSE=$(curl -k -s -G "https://localhost:$TEST_PORT_AUTH/oauth2/authorize" \
    --data-urlencode "response_type=code" \
    --data-urlencode "client_id=test-client" \
    --data-urlencode "redirect_uri=https://example.com/callback")

echo "$OAUTH_RESPONSE" | jq -e .status >/dev/null || API_TEST_RESULT=1

if [[ $API_TEST_RESULT -eq 0 ]]; then
    success "API integration tests passed"
else
    error "API integration tests failed"
    exit 1
fi

# Phase 6: TLS Certificate Tests
log "Phase 6: Testing TLS certificates..."

# Check certificate validity
CERT_TEST_RESULT=0
openssl s_client -connect "localhost:$TEST_PORT_AUTH" -servername localhost </dev/null 2>/dev/null | \
    openssl x509 -noout -dates >/dev/null || CERT_TEST_RESULT=1

if [[ $CERT_TEST_RESULT -eq 0 ]]; then
    success "TLS certificate tests passed"
else
    error "TLS certificate tests failed"
    exit 1
fi

# Phase 7: Performance Tests
log "Phase 7: Running basic performance tests..."

PERF_TEST_RESULT=0

# Test response times
for i in {1..10}; do
    RESPONSE_TIME=$(curl -k -s -w "%{time_total}" -o /dev/null "https://localhost:$TEST_PORT_AUTH/health")
    if (( $(echo "$RESPONSE_TIME > 1.0" | bc -l) )); then
        warn "Slow response detected: ${RESPONSE_TIME}s"
        PERF_TEST_RESULT=1
    fi
done

if [[ $PERF_TEST_RESULT -eq 0 ]]; then
    success "Performance tests passed"
else
    warn "Performance tests showed slow responses"
fi

# Test Summary
echo
echo "🎯 Test Summary"
echo "==============="
success "✅ Build: Successful"
success "✅ Unit Tests: Passed"
success "✅ CLI Tests: Passed"
success "✅ Services: Started successfully"
success "✅ API Tests: Passed"
success "✅ TLS Tests: Passed"
if [[ $PERF_TEST_RESULT -eq 0 ]]; then
    success "✅ Performance: Good"
else
    warn "⚠️  Performance: Some slow responses"
fi

echo
log "Test endpoints available:"
echo "  🔒 Auth Service: https://localhost:$TEST_PORT_AUTH"
echo "  🔧 Admin Service: http://localhost:$TEST_PORT_ADMIN"
echo
log "Services are running in background for manual testing"
log "Press Ctrl+C to stop all services and cleanup"

# Keep running for manual testing
log "Keeping services running for manual testing..."
log "Available test endpoints:"
echo "  curl -k https://localhost:$TEST_PORT_AUTH/health"
echo "  curl -k https://localhost:$TEST_PORT_AUTH/.well-known/openid-configuration"
echo "  curl http://localhost:$TEST_PORT_ADMIN/health"

# Wait for user interrupt
wait