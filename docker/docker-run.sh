#!/bin/bash
# Docker Compose Wrapper für UM-OIC mit Podman-Kompatibilität

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() { echo -e "${BLUE}[DOCKER]${NC} $1"; }
success() { echo -e "${GREEN}✅${NC} $1"; }
fail() { echo -e "${RED}❌${NC} $1"; }
warn() { echo -e "${YELLOW}⚠️${NC} $1"; }

# Setup Podman socket for Docker compatibility
setup_podman() {
    if ! systemctl --user is-active --quiet podman.socket; then
        log "Starting Podman socket for Docker compatibility..."
        systemctl --user start podman.socket
        success "Podman socket started"
    fi

    export DOCKER_HOST=unix:///run/user/1000/podman/podman.sock
    log "Using Podman socket: $DOCKER_HOST"
}

# Check prerequisites
check_requirements() {
    if ! command -v podman &> /dev/null; then
        fail "Podman is not installed"
        exit 1
    fi

    if ! command -v docker-compose &> /dev/null; then
        fail "Docker Compose is not installed"
        exit 1
    fi

    success "Requirements satisfied (Podman + Docker Compose)"
}

# Show usage
usage() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  build     Build all Docker images"
    echo "  up        Start all services"
    echo "  down      Stop all services"
    echo "  logs      Show service logs"
    echo "  status    Show service status"
    echo "  test      Run container tests"
    echo "  clean     Clean up everything"
    echo ""
    echo "Examples:"
    echo "  $0 build     # Build images"
    echo "  $0 up        # Start services"
    echo "  $0 test      # Test everything"
}

# Build images
build_images() {
    log "Building UM-OIC Docker images..."
    docker-compose -f docker-compose.private.yml build
    success "All images built successfully"
}

# Start services
start_services() {
    log "Starting UM-OIC services..."
    docker-compose -f docker-compose.private.yml up -d

    log "Waiting for services to start..."
    sleep 10

    # Check health
    if curl -k -s https://localhost:8443/health >/dev/null 2>&1; then
        success "Auth service is healthy"
    else
        warn "Auth service may still be starting"
    fi

    if curl -s http://localhost:8444/health >/dev/null 2>&1; then
        success "Admin service is healthy"
    else
        warn "Admin service may still be starting"
    fi

    echo ""
    log "Services are running:"
    echo "🔒 Auth Service:  https://localhost:8443"
    echo "🔧 Admin Service: http://localhost:8444"
    echo "💾 Redis:         localhost:6379"
}

# Stop services
stop_services() {
    log "Stopping UM-OIC services..."
    docker-compose -f docker-compose.private.yml down
    success "All services stopped"
}

# Show logs
show_logs() {
    log "Service logs:"
    docker-compose -f docker-compose.private.yml logs --tail=50
}

# Show status
show_status() {
    log "Service status:"
    docker-compose -f docker-compose.private.yml ps

    echo ""
    log "Container resources:"
    podman stats --no-stream --format "table {{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}"
}

# Run tests
run_tests() {
    log "Running container tests..."

    # Test auth service
    if curl -k -s https://localhost:8443/health | jq -e '.status == "healthy"' >/dev/null 2>&1; then
        success "Auth service health check passed"
    else
        fail "Auth service health check failed"
    fi

    # Test admin service
    local admin_status=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8444/health 2>/dev/null || echo "000")
    if [[ "$admin_status" == "200" || "$admin_status" == "401" ]]; then
        success "Admin service is responding (status: $admin_status)"
    else
        fail "Admin service not responding (status: $admin_status)"
    fi

    # Test OIDC
    if curl -k -s https://localhost:8443/.well-known/openid-configuration | jq -e '.issuer' >/dev/null 2>&1; then
        success "OIDC Discovery working"
    else
        fail "OIDC Discovery failed"
    fi

    log "Test completed"
}

# Clean up everything
cleanup() {
    log "Cleaning up Docker environment..."
    docker-compose -f docker-compose.private.yml down -v
    podman system prune -f
    success "Cleanup completed"
}

# Main execution
main() {
    check_requirements
    setup_podman

    case "${1:-help}" in
        build)
            build_images
            ;;
        up|start)
            start_services
            ;;
        down|stop)
            stop_services
            ;;
        logs)
            show_logs
            ;;
        status)
            show_status
            ;;
        test)
            run_tests
            ;;
        clean)
            cleanup
            ;;
        help|--help|-h)
            usage
            ;;
        *)
            echo "Unknown command: $1"
            echo ""
            usage
            exit 1
            ;;
    esac
}

main "$@"