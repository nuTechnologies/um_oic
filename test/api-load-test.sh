#!/bin/bash

# UM-OIC Load Test
# Tests multiple concurrent API calls for performance
# Nach RULEZ: Explizite Messung ohne Fallbacks

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
AUTH_SERVICE_URL="https://localhost:8443"
ADMIN_SERVICE_URL="https://localhost:8445"
TEST_EMAIL="admin@example.com"
TEST_PASSWORD="admin123"
OUTPUT_DIR="./test-results"
TIMESTAMP=$(date "+%Y%m%d_%H%M%S")
LOG_FILE="$OUTPUT_DIR/load_test_$TIMESTAMP.log"

# Load test parameters
CONCURRENT_USERS=5
REQUESTS_PER_USER=10
MAX_RESPONSE_TIME=2000  # milliseconds

log() {
    echo -e "${BLUE}[LOAD-TEST]${NC} $1" | tee -a "$LOG_FILE"
}

success() {
    echo -e "${GREEN}✅${NC} $1" | tee -a "$LOG_FILE"
}

error() {
    echo -e "${RED}❌${NC} $1" | tee -a "$LOG_FILE"
}

warn() {
    echo -e "${YELLOW}⚠️${NC} $1" | tee -a "$LOG_FILE"
}

# Setup
setup() {
    log "Setting up load test environment..."
    mkdir -p "$OUTPUT_DIR"

    # Get auth token once for all tests
    local login_data="{\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\"}"
    local response=$(curl -k -s -X POST \
        -H "Content-Type: application/json" \
        -d "$login_data" \
        "$AUTH_SERVICE_URL/api/auth/login")

    local access_token=$(echo "$response" | jq -r '.access_token // empty' 2>/dev/null)

    if [ -n "$access_token" ] && [ "$access_token" != "null" ]; then
        echo "$access_token" > /tmp/load_test_token.txt
        success "Authentication successful for load testing"
    else
        error "Authentication failed - cannot proceed with load test"
        exit 1
    fi
}

# Single request with timing
timed_request() {
    local endpoint="$1"
    local user_id="$2"
    local request_id="$3"

    local token=$(cat /tmp/load_test_token.txt)
    local start_time=$(date +%s%3N)

    local response=$(curl -k -s -w '%{http_code}' \
        -H "Authorization: Bearer $token" \
        "$endpoint" 2>/dev/null)

    local end_time=$(date +%s%3N)
    local duration=$((end_time - start_time))

    local status_code="${response: -3}"

    echo "$user_id,$request_id,$endpoint,$status_code,$duration" >> "$OUTPUT_DIR/load_test_raw_$TIMESTAMP.csv"

    if [ "$status_code" = "200" ] && [ "$duration" -lt "$MAX_RESPONSE_TIME" ]; then
        return 0
    else
        return 1
    fi
}

# Simulate single user
simulate_user() {
    local user_id="$1"
    local success_count=0
    local total_requests=0

    log "Starting user $user_id simulation..."

    # Test different endpoints
    local endpoints=(
        "$ADMIN_SERVICE_URL/api/users"
        "$ADMIN_SERVICE_URL/api/organizations"
        "$ADMIN_SERVICE_URL/api/clients"
        "$ADMIN_SERVICE_URL/stats/users"
        "$ADMIN_SERVICE_URL/api/system/status"
    )

    for ((i=1; i<=REQUESTS_PER_USER; i++)); do
        local endpoint_index=$((i % ${#endpoints[@]}))
        local endpoint="${endpoints[$endpoint_index]}"

        if timed_request "$endpoint" "$user_id" "$i"; then
            ((success_count++))
        fi
        ((total_requests++))

        # Small delay between requests
        sleep 0.1
    done

    echo "$user_id,$success_count,$total_requests" >> "$OUTPUT_DIR/load_test_users_$TIMESTAMP.csv"
    log "User $user_id completed: $success_count/$total_requests successful"
}

# Run concurrent load test
run_load_test() {
    log "Starting load test with $CONCURRENT_USERS concurrent users..."
    log "Each user will make $REQUESTS_PER_USER requests"

    # Initialize CSV files
    echo "user_id,request_id,endpoint,status_code,response_time_ms" > "$OUTPUT_DIR/load_test_raw_$TIMESTAMP.csv"
    echo "user_id,success_count,total_requests" > "$OUTPUT_DIR/load_test_users_$TIMESTAMP.csv"

    local start_time=$(date +%s)

    # Start concurrent users
    local pids=()
    for ((user=1; user<=CONCURRENT_USERS; user++)); do
        simulate_user "$user" &
        pids+=($!)
    done

    # Wait for all users to complete
    for pid in "${pids[@]}"; do
        wait "$pid"
    done

    local end_time=$(date +%s)
    local total_duration=$((end_time - start_time))

    log "Load test completed in ${total_duration}s"

    # Analyze results
    analyze_results
}

# Analyze test results
analyze_results() {
    log "Analyzing load test results..."

    local raw_file="$OUTPUT_DIR/load_test_raw_$TIMESTAMP.csv"
    local users_file="$OUTPUT_DIR/load_test_users_$TIMESTAMP.csv"

    if [ ! -f "$raw_file" ] || [ ! -f "$users_file" ]; then
        error "Result files not found"
        return 1
    fi

    # Calculate statistics
    local total_requests=$(tail -n +2 "$raw_file" | wc -l)
    local successful_requests=$(tail -n +2 "$raw_file" | awk -F',' '$4 == 200 { print $0 }' | wc -l)
    local failed_requests=$((total_requests - successful_requests))

    # Response time statistics
    local avg_response_time=$(tail -n +2 "$raw_file" | awk -F',' '$4 == 200 { sum += $5; count++ } END { if (count > 0) print int(sum/count); else print 0 }')
    local max_response_time=$(tail -n +2 "$raw_file" | awk -F',' '$4 == 200 { if ($5 > max) max = $5 } END { print max+0 }')
    local min_response_time=$(tail -n +2 "$raw_file" | awk -F',' '$4 == 200 { if (min == "" || $5 < min) min = $5 } END { print min+0 }')

    # Requests per second
    local duration_seconds=$(tail -n +2 "$raw_file" | awk -F',' 'BEGIN{min_time=99999999999; max_time=0} {if($5<min_time) min_time=$5; if($5>max_time) max_time=$5} END{print int((max_time-min_time)/1000)}')
    if [ "$duration_seconds" -eq 0 ]; then
        duration_seconds=1
    fi
    local requests_per_second=$((total_requests / duration_seconds))

    # Output results
    echo ""
    echo "📊 Load Test Results"
    echo "===================="
    echo "Total Requests: $total_requests"
    echo "Successful: $successful_requests"
    echo "Failed: $failed_requests"
    echo "Success Rate: $(( successful_requests * 100 / total_requests ))%"
    echo ""
    echo "Response Times (ms):"
    echo "  Average: ${avg_response_time}ms"
    echo "  Minimum: ${min_response_time}ms"
    echo "  Maximum: ${max_response_time}ms"
    echo ""
    echo "Throughput:"
    echo "  Requests per second: $requests_per_second"
    echo "  Concurrent users: $CONCURRENT_USERS"
    echo ""

    # Performance assessment
    if [ "$successful_requests" -eq "$total_requests" ] && [ "$avg_response_time" -lt 1000 ]; then
        success "Performance test PASSED - All requests successful with good response times"
    elif [ "$avg_response_time" -gt "$MAX_RESPONSE_TIME" ]; then
        warn "Performance test WARNING - Average response time too high (${avg_response_time}ms > ${MAX_RESPONSE_TIME}ms)"
    elif [ "$failed_requests" -gt 0 ]; then
        error "Performance test FAILED - $failed_requests requests failed"
    else
        success "Performance test PASSED with acceptable performance"
    fi

    # Generate summary JSON
    cat > "$OUTPUT_DIR/load_test_summary_$TIMESTAMP.json" << EOF
{
    "timestamp": "$TIMESTAMP",
    "configuration": {
        "concurrent_users": $CONCURRENT_USERS,
        "requests_per_user": $REQUESTS_PER_USER,
        "max_response_time_ms": $MAX_RESPONSE_TIME
    },
    "results": {
        "total_requests": $total_requests,
        "successful_requests": $successful_requests,
        "failed_requests": $failed_requests,
        "success_rate_percent": $(( successful_requests * 100 / total_requests )),
        "response_times": {
            "average_ms": $avg_response_time,
            "minimum_ms": $min_response_time,
            "maximum_ms": $max_response_time
        },
        "throughput": {
            "requests_per_second": $requests_per_second
        }
    }
}
EOF

    log "Detailed results saved to:"
    log "  Raw data: $raw_file"
    log "  User summary: $users_file"
    log "  JSON summary: $OUTPUT_DIR/load_test_summary_$TIMESTAMP.json"
}

# Cleanup
cleanup() {
    log "Cleaning up load test environment..."
    rm -f /tmp/load_test_token.txt
}

# Main function
main() {
    echo "🚀 UM-OIC Load Test"
    echo "==================="
    echo "Target: $ADMIN_SERVICE_URL"
    echo "Concurrent Users: $CONCURRENT_USERS"
    echo "Requests per User: $REQUESTS_PER_USER"
    echo "Max Response Time: ${MAX_RESPONSE_TIME}ms"
    echo ""

    setup
    run_load_test
    cleanup
}

# Trap cleanup on exit
trap cleanup EXIT

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -u|--users)
            CONCURRENT_USERS="$2"
            shift 2
            ;;
        -r|--requests)
            REQUESTS_PER_USER="$2"
            shift 2
            ;;
        -t|--timeout)
            MAX_RESPONSE_TIME="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -u, --users     Number of concurrent users (default: $CONCURRENT_USERS)"
            echo "  -r, --requests  Requests per user (default: $REQUESTS_PER_USER)"
            echo "  -t, --timeout   Max response time in ms (default: $MAX_RESPONSE_TIME)"
            echo "  -h, --help      Show this help"
            echo ""
            exit 0
            ;;
        *)
            error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Run main function
main "$@"