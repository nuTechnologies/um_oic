#!/bin/bash
# Service Manager for Tests

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() { echo -e "${BLUE}[SVC-MGR]${NC} $1"; }
success() { echo -e "${GREEN}✅${NC} $1"; }
fail() { echo -e "${RED}❌${NC} $1"; }
warn() { echo -e "${YELLOW}⚠️${NC} $1"; }

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

start_services() {
    log "Starting services for testing..."

    # Kill any existing services
    pkill -f "auth-service" 2>/dev/null || true
    pkill -f "admin-service" 2>/dev/null || true
    sleep 2

    # Remove PID files
    rm -f "$PROJECT_ROOT"/*.pid

    # Build services
    log "Building services..."
    cd "$PROJECT_ROOT"
    cargo build || {
        fail "Build failed"
        return 1
    }

    # Start auth service
    log "Starting auth service on https://localhost:8443..."
    cd "$PROJECT_ROOT/auth-service"
    RUST_LOG=info cargo run -- \
        --tls-enable \
        --data-dir ../data \
        --config config.toml \
        > /tmp/auth-service.log 2>&1 &

    AUTH_PID=$!
    echo $AUTH_PID > /tmp/auth-service.pid

    # Start admin service
    log "Starting admin service on http://localhost:8444..."
    cd "$PROJECT_ROOT/admin-service"
    RUST_LOG=info cargo run -- \
        --data-dir ../data \
        --config config.toml \
        > /tmp/admin-service.log 2>&1 &

    ADMIN_PID=$!
    echo $ADMIN_PID > /tmp/admin-service.pid

    # Wait for services to start
    log "Waiting for services to initialize..."
    sleep 10

    # Check if services are running
    if ! kill -0 $AUTH_PID 2>/dev/null; then
        fail "Auth service failed to start"
        cat /tmp/auth-service.log
        return 1
    fi

    if ! kill -0 $ADMIN_PID 2>/dev/null; then
        fail "Admin service failed to start"
        cat /tmp/admin-service.log
        return 1
    fi

    # Test basic connectivity
    if curl -k -s --max-time 5 https://localhost:8443/health >/dev/null; then
        success "Auth service is responding"
    else
        fail "Auth service is not responding"
        return 1
    fi

    if curl -s --max-time 5 http://localhost:8444/health >/dev/null; then
        success "Admin service is responding"
    else
        fail "Admin service is not responding"
        return 1
    fi

    success "All services started successfully"
    return 0
}

stop_services() {
    log "Stopping services..."

    if [ -f /tmp/auth-service.pid ]; then
        AUTH_PID=$(cat /tmp/auth-service.pid)
        if kill -0 $AUTH_PID 2>/dev/null; then
            kill $AUTH_PID
            wait $AUTH_PID 2>/dev/null || true
        fi
        rm -f /tmp/auth-service.pid
    fi

    if [ -f /tmp/admin-service.pid ]; then
        ADMIN_PID=$(cat /tmp/admin-service.pid)
        if kill -0 $ADMIN_PID 2>/dev/null; then
            kill $ADMIN_PID
            wait $ADMIN_PID 2>/dev/null || true
        fi
        rm -f /tmp/admin-service.pid
    fi

    # Cleanup any remaining processes
    pkill -f "auth-service" 2>/dev/null || true
    pkill -f "admin-service" 2>/dev/null || true

    success "Services stopped"
}

show_logs() {
    echo "=== Auth Service Logs ==="
    if [ -f /tmp/auth-service.log ]; then
        tail -20 /tmp/auth-service.log
    else
        echo "No auth service log found"
    fi

    echo -e "\n=== Admin Service Logs ==="
    if [ -f /tmp/admin-service.log ]; then
        tail -20 /tmp/admin-service.log
    else
        echo "No admin service log found"
    fi
}

case "$1" in
    start)
        start_services
        ;;
    stop)
        stop_services
        ;;
    restart)
        stop_services
        sleep 2
        start_services
        ;;
    logs)
        show_logs
        ;;
    *)
        echo "Usage: $0 {start|stop|restart|logs}"
        exit 1
        ;;
esac