#!/bin/bash
# Performance Benchmarks for UM-OIC

set -e

# Test configuration
AUTH_URL="http://localhost:8080"
ADMIN_URL="http://localhost:8081"
CONCURRENT_USERS=10
REQUESTS_PER_USER=50

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() {
    echo -e "${BLUE}[BENCHMARK]${NC} $1"
}

success() {
    echo -e "${GREEN}✓${NC} $1"
}

error() {
    echo -e "${RED}✗${NC} $1"
}

# Check if Apache Bench is available
check_dependencies() {
    if ! command -v ab &> /dev/null; then
        error "Apache Bench (ab) is required but not installed"
        echo "Install with: sudo apt-get install apache2-utils"
        return 1
    fi

    if ! command -v jq &> /dev/null; then
        error "jq is required but not installed"
        echo "Install with: sudo apt-get install jq"
        return 1
    fi

    success "Dependencies available"
}

# Benchmark 1: Health Endpoint
benchmark_health() {
    log "Benchmarking health endpoints..."

    # Auth service health
    log "Auth service health check..."
    ab -n $((CONCURRENT_USERS * REQUESTS_PER_USER)) -c $CONCURRENT_USERS \
        "$AUTH_URL/health" 2>/dev/null | \
        grep -E "(Requests per second|Time per request)" | \
        sed 's/^/  /'

    # Admin service health
    log "Admin service health check..."
    ab -n $((CONCURRENT_USERS * REQUESTS_PER_USER)) -c $CONCURRENT_USERS \
        "$ADMIN_URL/health" 2>/dev/null | \
        grep -E "(Requests per second|Time per request)" | \
        sed 's/^/  /'

    success "Health endpoint benchmarks completed"
}

# Benchmark 2: OAuth2 Discovery
benchmark_discovery() {
    log "Benchmarking OAuth2 discovery..."

    ab -n $((CONCURRENT_USERS * 20)) -c $CONCURRENT_USERS \
        "$AUTH_URL/.well-known/openid_configuration" 2>/dev/null | \
        grep -E "(Requests per second|Time per request)" | \
        sed 's/^/  /'

    success "Discovery endpoint benchmark completed"
}

# Benchmark 3: Login Performance
benchmark_login() {
    log "Benchmarking login performance..."

    # Create temporary file with login data
    local login_data='{"email":"admin@test.local","password":"testpassword123"}'
    echo "$login_data" > /tmp/login_data.json

    # Use curl for POST requests (ab doesn't handle JSON POST well)
    log "Running $((CONCURRENT_USERS * 10)) login requests..."

    local start_time=$(date +%s.%N)
    local pids=()

    for ((i=1; i<=CONCURRENT_USERS; i++)); do
        {
            for ((j=1; j<=10; j++)); do
                curl -s \
                    -H "Content-Type: application/json" \
                    -d @/tmp/login_data.json \
                    "$AUTH_URL/auth/login" > /dev/null
            done
        } &
        pids+=($!)
    done

    # Wait for all processes to complete
    for pid in "${pids[@]}"; do
        wait $pid
    done

    local end_time=$(date +%s.%N)
    local duration=$(echo "$end_time - $start_time" | bc -l)
    local total_requests=$((CONCURRENT_USERS * 10))
    local rps=$(echo "scale=2; $total_requests / $duration" | bc -l)

    echo "  Login Performance:"
    echo "    Total requests: $total_requests"
    echo "    Duration: ${duration}s"
    echo "    Requests per second: $rps"

    rm -f /tmp/login_data.json
    success "Login benchmark completed"
}

# Benchmark 4: Admin API Performance
benchmark_admin_api() {
    log "Benchmarking admin API performance..."

    # Get admin token first
    local login_data='{"email":"admin@test.local","password":"testpassword123"}'
    local auth_response=$(curl -s \
        -H "Content-Type: application/json" \
        -d "$login_data" \
        "$AUTH_URL/auth/login")

    local admin_token=$(echo "$auth_response" | jq -r ".access_token")

    if [ "$admin_token" = "null" ] || [ "$admin_token" = "" ]; then
        error "Failed to get admin token for benchmark"
        return 1
    fi

    # Create temporary header file
    echo "Authorization: Bearer $admin_token" > /tmp/auth_header.txt

    # Benchmark user listing
    log "Users API endpoint..."
    ab -n $((CONCURRENT_USERS * 20)) -c $CONCURRENT_USERS \
        -H "Authorization: Bearer $admin_token" \
        "$ADMIN_URL/api/users" 2>/dev/null | \
        grep -E "(Requests per second|Time per request)" | \
        sed 's/^/  /'

    # Benchmark system status
    log "System status endpoint..."
    ab -n $((CONCURRENT_USERS * 30)) -c $CONCURRENT_USERS \
        -H "Authorization: Bearer $admin_token" \
        "$ADMIN_URL/api/system/status" 2>/dev/null | \
        grep -E "(Requests per second|Time per request)" | \
        sed 's/^/  /'

    rm -f /tmp/auth_header.txt
    success "Admin API benchmark completed"
}

# Benchmark 5: Memory and CPU Usage
benchmark_resources() {
    log "Monitoring resource usage..."

    # Get process IDs
    local auth_pid=$(pgrep -f auth-service | head -1)
    local admin_pid=$(pgrep -f admin-service | head -1)

    if [ -z "$auth_pid" ] || [ -z "$admin_pid" ]; then
        error "Services not running, cannot monitor resources"
        return 1
    fi

    # Monitor for 30 seconds during load
    log "Starting resource monitoring (30 seconds)..."

    # Background monitoring
    {
        for i in {1..30}; do
            local auth_mem=$(ps -p $auth_pid -o rss= 2>/dev/null || echo "0")
            local admin_mem=$(ps -p $admin_pid -o rss= 2>/dev/null || echo "0")
            local auth_cpu=$(ps -p $auth_pid -o %cpu= 2>/dev/null || echo "0")
            local admin_cpu=$(ps -p $admin_pid -o %cpu= 2>/dev/null || echo "0")

            echo "$i,$auth_mem,$admin_mem,$auth_cpu,$admin_cpu" >> /tmp/resource_usage.csv
            sleep 1
        done
    } &
    local monitor_pid=$!

    # Generate load during monitoring
    ab -n 1000 -c 20 "$AUTH_URL/health" > /dev/null 2>&1 &
    ab -n 1000 -c 20 "$ADMIN_URL/health" > /dev/null 2>&1 &

    # Wait for monitoring to complete
    wait $monitor_pid

    # Analyze results
    if [ -f /tmp/resource_usage.csv ]; then
        local max_auth_mem=$(cut -d',' -f2 /tmp/resource_usage.csv | sort -n | tail -1)
        local max_admin_mem=$(cut -d',' -f3 /tmp/resource_usage.csv | sort -n | tail -1)
        local avg_auth_cpu=$(cut -d',' -f4 /tmp/resource_usage.csv | awk '{sum+=$1} END {print sum/NR}')
        local avg_admin_cpu=$(cut -d',' -f5 /tmp/resource_usage.csv | awk '{sum+=$1} END {print sum/NR}')

        echo "  Resource Usage Summary:"
        echo "    Auth Service - Max Memory: ${max_auth_mem}KB, Avg CPU: ${avg_auth_cpu}%"
        echo "    Admin Service - Max Memory: ${max_admin_mem}KB, Avg CPU: ${avg_admin_cpu}%"

        rm -f /tmp/resource_usage.csv
    fi

    success "Resource monitoring completed"
}

# Benchmark 6: Latency Analysis
benchmark_latency() {
    log "Analyzing response latency..."

    # Test various endpoints with different request sizes
    local endpoints=(
        "$AUTH_URL/health"
        "$AUTH_URL/.well-known/openid_configuration"
        "$ADMIN_URL/health"
    )

    for endpoint in "${endpoints[@]}"; do
        log "Testing latency for $(basename $endpoint)..."

        # Use curl to measure timing
        local total_time=0
        local requests=50

        for ((i=1; i<=requests; i++)); do
            local time=$(curl -s -w "%{time_total}" -o /dev/null "$endpoint")
            total_time=$(echo "$total_time + $time" | bc -l)
        done

        local avg_time=$(echo "scale=3; $total_time / $requests" | bc -l)
        echo "  Average response time: ${avg_time}s"
    done

    success "Latency analysis completed"
}

# Main benchmark runner
main() {
    log "UM-OIC Performance Benchmarks"
    echo "=============================="

    if ! check_dependencies; then
        return 1
    fi

    log "Configuration:"
    echo "  Concurrent Users: $CONCURRENT_USERS"
    echo "  Requests per User: $REQUESTS_PER_USER"
    echo "  Auth URL: $AUTH_URL"
    echo "  Admin URL: $ADMIN_URL"
    echo ""

    # Run all benchmarks
    benchmark_health
    echo ""

    benchmark_discovery
    echo ""

    benchmark_login
    echo ""

    benchmark_admin_api
    echo ""

    benchmark_resources
    echo ""

    benchmark_latency
    echo ""

    success "All benchmarks completed!"
    echo ""
    echo "Note: For production benchmarking, consider using tools like:"
    echo "  - wrk (https://github.com/wg/wrk)"
    echo "  - artillery (https://artillery.io/)"
    echo "  - JMeter (https://jmeter.apache.org/)"
}

# Show usage
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  -c USERS    Set concurrent users (default: $CONCURRENT_USERS)"
    echo "  -n REQUESTS Set requests per user (default: $REQUESTS_PER_USER)"
    echo "  -h, --help  Show this help message"
    echo ""
    echo "Environment variables:"
    echo "  AUTH_URL    Auth service URL (default: $AUTH_URL)"
    echo "  ADMIN_URL   Admin service URL (default: $ADMIN_URL)"
    exit 0
fi

# Parse command line options
while [[ $# -gt 0 ]]; do
    case $1 in
        -c)
            CONCURRENT_USERS="$2"
            shift 2
            ;;
        -n)
            REQUESTS_PER_USER="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

main "$@"