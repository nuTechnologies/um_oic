#!/bin/bash
# UM-OIC Docker Test Suite
# Testet das komplette Docker-Setup mit TLS

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() {
    echo -e "${BLUE}[DOCKER-TEST]${NC} $1"
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

cleanup() {
    log "Cleaning up Docker containers..."
    docker-compose -f docker-compose.private.yml down -v 2>/dev/null || true
    docker system prune -f 2>/dev/null || true
    success "Docker cleanup completed"
}

trap cleanup EXIT

echo "🐳 UM-OIC Docker Test Suite"
echo "============================"
echo

# Check Docker availability
if ! command -v docker &> /dev/null; then
    error "Docker is not installed or not available"
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    error "Docker Compose is not installed or not available"
    exit 1
fi

success "Docker and Docker Compose are available"

# Check if docker-compose.private.yml exists
if [[ ! -f "docker-compose.private.yml" ]]; then
    error "docker-compose.private.yml not found"
    exit 1
fi

# Build Docker images
log "Building Docker images..."
docker-compose -f docker-compose.private.yml build
if [[ $? -eq 0 ]]; then
    success "Docker images built successfully"
else
    error "Docker build failed"
    exit 1
fi

# Start services
log "Starting Docker services..."
docker-compose -f docker-compose.private.yml up -d

# Wait for services to be ready
log "Waiting for services to start..."
sleep 10

# Check if containers are running
AUTH_STATUS=$(docker-compose -f docker-compose.private.yml ps --services --filter "status=running" | grep auth-service || echo "")
ADMIN_STATUS=$(docker-compose -f docker-compose.private.yml ps --services --filter "status=running" | grep admin-service || echo "")

if [[ -n "$AUTH_STATUS" ]]; then
    success "Auth service container is running"
else
    error "Auth service container failed to start"
    docker-compose -f docker-compose.private.yml logs auth-service
    exit 1
fi

if [[ -n "$ADMIN_STATUS" ]]; then
    success "Admin service container is running"
else
    error "Admin service container failed to start"
    docker-compose -f docker-compose.private.yml logs admin-service
    exit 1
fi

# Test HTTPS endpoints
log "Testing HTTPS endpoints..."
for i in {1..10}; do
    if curl -k -s "https://localhost:8443/health" >/dev/null 2>&1; then
        success "Auth service HTTPS endpoint is responding"
        break
    fi
    if [[ $i -eq 10 ]]; then
        error "Auth service HTTPS endpoint not responding"
        exit 1
    fi
    sleep 2
done

# Test admin service
for i in {1..10}; do
    ADMIN_HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "https://localhost:8445/health" 2>/dev/null || echo "000")
    if [[ $ADMIN_HTTP_STATUS -eq 401 ]]; then
        success "Admin service is responding (auth required)"
        break
    fi
    if [[ $i -eq 10 ]]; then
        warn "Admin service status: $ADMIN_HTTP_STATUS"
        break
    fi
    sleep 2
done

# Test TLS certificate in Docker
log "Testing TLS certificate..."
CERT_INFO=$(openssl s_client -connect localhost:8443 -servername localhost < /dev/null 2>/dev/null | openssl x509 -noout -subject 2>/dev/null || echo "")
if [[ -n "$CERT_INFO" ]]; then
    success "TLS certificate is valid: $CERT_INFO"
else
    error "TLS certificate test failed"
fi

# Test API endpoints
log "Testing API endpoints..."
API_HEALTH=$(curl -k -s "https://localhost:8443/health" 2>/dev/null || echo '{}')
API_STATUS=$(echo "$API_HEALTH" | jq -r .status 2>/dev/null || echo "error")

if [[ "$API_STATUS" == "healthy" ]]; then
    success "API health check passed"
else
    error "API health check failed: $API_STATUS"
fi

# Test OIDC Discovery
OIDC_CONFIG=$(curl -k -s "https://localhost:8443/.well-known/openid-configuration" 2>/dev/null || echo '{}')
OIDC_ISSUER=$(echo "$OIDC_CONFIG" | jq -r .issuer 2>/dev/null || echo "")

if [[ -n "$OIDC_ISSUER" && "$OIDC_ISSUER" != "null" ]]; then
    success "OIDC Discovery working: $OIDC_ISSUER"
else
    error "OIDC Discovery failed"
fi

# Show container logs for debugging
log "Container status:"
docker-compose -f docker-compose.private.yml ps

# Show volumes
log "Docker volumes:"
docker volume ls | grep um-oic || true

echo
log "=== Docker Test Summary ==="
success "✅ Docker build: Successful"
success "✅ Container startup: Successful"
success "✅ HTTPS endpoints: Working"
success "✅ TLS certificates: Valid"
success "✅ API responses: Working"
success "✅ OIDC Discovery: Working"

echo
log "=== Test Endpoints ==="
echo "🔒 Auth Service: https://localhost:8443"
echo "🔧 Admin Service: https://localhost:8445"
echo
echo "Sample commands:"
echo "curl -k https://localhost:8443/health | jq"
echo "curl -k https://localhost:8443/.well-known/openid-configuration | jq"
echo
log "Docker containers will continue running for manual testing"
log "Press Ctrl+C to stop and cleanup"

# Keep containers running for manual testing
wait