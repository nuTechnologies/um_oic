#!/bin/bash
# UM-OIC Test Runner

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test directories
TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$TEST_DIR")"

# Default test ports
AUTH_PORT=8080
ADMIN_PORT=8081

log() {
    echo -e "${BLUE}[$(date +'%H:%M:%S')]${NC} $1"
}

success() {
    echo -e "${GREEN}✓${NC} $1"
}

error() {
    echo -e "${RED}✗${NC} $1"
}

warn() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Check if services are running
check_service() {
    local port=$1
    local name=$2

    if curl -s -f "http://localhost:$port/health" > /dev/null 2>&1; then
        success "$name service is running on port $port"
        return 0
    else
        error "$name service is not running on port $port"
        return 1
    fi
}

# Start test services
start_test_services() {
    log "Starting test services..."

    # Kill existing processes
    pkill -f "auth-service" || true
    pkill -f "admin-service" || true
    sleep 2

    # Start auth-service in test mode
    cd "$PROJECT_ROOT"
    RUST_LOG=debug PORT=$AUTH_PORT DATA_DIR="$TEST_DIR/data/auth" \
        ./target/debug/auth-service &
    AUTH_PID=$!

    # Start admin-service in test mode
    RUST_LOG=debug PORT=$ADMIN_PORT DATA_DIR="$TEST_DIR/data/admin" \
        AUTH_SERVICE_URL="http://localhost:$AUTH_PORT" \
        ./target/debug/admin-service &
    ADMIN_PID=$!

    # Wait for services to start
    sleep 3

    # Verify services are running
    if ! check_service $AUTH_PORT "Auth"; then
        error "Failed to start auth-service"
        cleanup_services
        exit 1
    fi

    if ! check_service $ADMIN_PORT "Admin"; then
        error "Failed to start admin-service"
        cleanup_services
        exit 1
    fi
}

# Stop test services
cleanup_services() {
    log "Stopping test services..."

    if [ ! -z "$AUTH_PID" ]; then
        kill $AUTH_PID 2>/dev/null || true
    fi

    if [ ! -z "$ADMIN_PID" ]; then
        kill $ADMIN_PID 2>/dev/null || true
    fi

    pkill -f "auth-service" || true
    pkill -f "admin-service" || true

    # Clean test data
    rm -rf "$TEST_DIR/data/auth" "$TEST_DIR/data/admin"
}

# Setup test data
setup_test_data() {
    log "Setting up test data..."

    mkdir -p "$TEST_DIR/data/auth" "$TEST_DIR/data/admin"

    # Copy test fixtures
    if [ -f "$TEST_DIR/fixtures/users.json" ]; then
        cp "$TEST_DIR/fixtures/"* "$TEST_DIR/data/auth/"
        cp "$TEST_DIR/fixtures/"* "$TEST_DIR/data/admin/"
    fi
}

# Run specific test category
run_tests() {
    local category=${1:-"all"}

    case $category in
        "auth-service"|"auth")
            log "Running auth-service tests..."
            bash "$TEST_DIR/auth-service/test-auth.sh"
            ;;
        "admin-service"|"admin")
            log "Running admin-service tests..."
            bash "$TEST_DIR/admin-service/test-admin.sh"
            ;;
        "integration")
            log "Running integration tests..."
            bash "$TEST_DIR/integration/test-integration.sh"
            ;;
        "all"|*)
            log "Running all tests..."
            bash "$TEST_DIR/auth-service/test-auth.sh"
            bash "$TEST_DIR/admin-service/test-admin.sh"
            bash "$TEST_DIR/integration/test-integration.sh"
            ;;
    esac
}

# Main execution
main() {
    log "UM-OIC Test Suite"
    echo "=================="

    # Ensure services are built
    if [ ! -f "$PROJECT_ROOT/target/debug/auth-service" ] || [ ! -f "$PROJECT_ROOT/target/debug/admin-service" ]; then
        log "Building services..."
        cd "$PROJECT_ROOT"
        cargo build
    fi

    # Setup trap for cleanup
    trap cleanup_services EXIT

    # Setup test environment
    setup_test_data
    start_test_services

    # Run tests
    run_tests "$1"

    success "All tests completed!"
}

# Show usage
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Usage: $0 [test-category]"
    echo ""
    echo "Test Categories:"
    echo "  auth-service    Run auth-service tests only"
    echo "  admin-service   Run admin-service tests only"
    echo "  integration     Run integration tests only"
    echo "  all             Run all tests (default)"
    echo ""
    echo "Options:"
    echo "  -h, --help      Show this help message"
    exit 0
fi

main "$1"