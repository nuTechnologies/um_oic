#!/bin/bash
# Admin Service Specific Tests

set -e

AUTH_BASE_URL="https://localhost:8443"
ADMIN_BASE_URL="https://localhost:8445"
TEST_EMAIL="admin@example.com"
TEST_PASSWORD="password123"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

log() { echo -e "${BLUE}[ADMIN-TEST]${NC} $1"; }
success() { echo -e "${GREEN}✅${NC} $1"; }
fail() { echo -e "${RED}❌${NC} $1"; }

# Get auth token first
get_auth_token() {
    log "Getting auth token..."
    local response=$(curl -k -s -X POST "$AUTH_BASE_URL/api/auth/login" \
        -H "Content-Type: application/json" \
        -d "{\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\"}" 2>/dev/null)

    echo "$response" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4
}

test_admin_api() {
    local name="$1"
    local method="$2"
    local endpoint="$3"
    local data="$4"
    local expected_status="$5"
    local auth_token="$6"

    log "Testing: $name"

    local cmd="curl -s -w '%{http_code}' -o /tmp/admin_test_response.json --max-time 10"

    if [ "$method" != "GET" ]; then
        cmd="$cmd -X $method"
    fi

    if [ -n "$data" ]; then
        cmd="$cmd -H 'Content-Type: application/json' -d '$data'"
    fi

    if [ -n "$auth_token" ]; then
        cmd="$cmd -H 'Authorization: Bearer $auth_token'"
    fi

    local status=$(eval "$cmd '$ADMIN_BASE_URL$endpoint'" 2>/dev/null || echo "000")

    if [ "$status" = "$expected_status" ]; then
        success "$name (Status: $status)"
        return 0
    else
        fail "$name - Expected: $expected_status, Got: $status"
        cat /tmp/admin_test_response.json 2>/dev/null || echo "No response"
        return 1
    fi
}

echo "🔧 Admin Service API Tests"
echo "==========================="

# Get authentication token
AUTH_TOKEN=$(get_auth_token)
if [ -z "$AUTH_TOKEN" ]; then
    fail "Could not get auth token - skipping authenticated tests"
fi

# Basic connectivity
test_admin_api "Health Check" "GET" "/health" "" "200"

# Static files
test_admin_api "Management Interface" "GET" "/mgmt/" "" "200"

# API endpoints without auth (should fail)
test_admin_api "Users Unauthorized" "GET" "/api/users" "" "401"
test_admin_api "Claims Unauthorized" "GET" "/api/claims" "" "401"
test_admin_api "Groups Unauthorized" "GET" "/api/groups" "" "401"
test_admin_api "Clients Unauthorized" "GET" "/api/clients" "" "401"
test_admin_api "Audit Unauthorized" "GET" "/api/audit" "" "401"

# API endpoints with auth (should succeed)
if [ -n "$AUTH_TOKEN" ]; then
    test_admin_api "Users Authorized" "GET" "/api/users" "" "200" "$AUTH_TOKEN"
    test_admin_api "Claims Authorized" "GET" "/api/claims" "" "200" "$AUTH_TOKEN"
    test_admin_api "Groups Authorized" "GET" "/api/groups" "" "200" "$AUTH_TOKEN"
    test_admin_api "Clients Authorized" "GET" "/api/clients" "" "200" "$AUTH_TOKEN"
    test_admin_api "Audit Authorized" "GET" "/api/audit" "" "200" "$AUTH_TOKEN"

    # Test user operations
    test_admin_api "Get Specific User" "GET" "/api/users/user-admin" "" "200" "$AUTH_TOKEN"

    # Test stats endpoint
    test_admin_api "System Stats" "GET" "/api/stats" "" "200" "$AUTH_TOKEN"
fi

echo "Admin Service tests completed!"