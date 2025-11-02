#!/bin/bash
# Auth Service Specific Tests

set -e

AUTH_BASE_URL="https://localhost:8443"
TEST_EMAIL="admin@example.com"
TEST_PASSWORD="password123"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

log() { echo -e "${BLUE}[AUTH-TEST]${NC} $1"; }
success() { echo -e "${GREEN}✅${NC} $1"; }
fail() { echo -e "${RED}❌${NC} $1"; }

test_api_call() {
    local name="$1"
    local method="$2"
    local endpoint="$3"
    local data="$4"
    local expected_status="$5"

    log "Testing: $name"

    local cmd="curl -k -s -w '%{http_code}' -o /tmp/auth_test_response.json --max-time 10"

    if [ "$method" != "GET" ]; then
        cmd="$cmd -X $method"
    fi

    if [ -n "$data" ]; then
        cmd="$cmd -H 'Content-Type: application/json' -d '$data'"
    fi

    local status=$(eval "$cmd '$AUTH_BASE_URL$endpoint'" 2>/dev/null || echo "000")

    if [ "$status" = "$expected_status" ]; then
        success "$name (Status: $status)"
        return 0
    else
        fail "$name - Expected: $expected_status, Got: $status"
        cat /tmp/auth_test_response.json 2>/dev/null || echo "No response"
        return 1
    fi
}

echo "🔐 Auth Service API Tests"
echo "========================="

# Basic connectivity
test_api_call "Health Check" "GET" "/health" "" "200"

# OIDC Discovery
test_api_call "OIDC Discovery" "GET" "/.well-known/openid-configuration" "" "200"

# Authentication endpoints
test_api_call "Login Valid" "POST" "/api/auth/login" "{\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\"}" "200"
test_api_call "Login Invalid" "POST" "/api/auth/login" "{\"email\":\"$TEST_EMAIL\",\"password\":\"wrong\"}" "200"
test_api_call "Login Missing Data" "POST" "/api/auth/login" "{}" "400"

# OAuth2 endpoints
test_api_call "OAuth2 Authorize" "GET" "/oauth2/authorize?response_type=code&client_id=test&redirect_uri=https://example.com" "" "200"

# Static files
test_api_call "Root Path" "GET" "/" "" "200"

echo "Auth Service tests completed!"