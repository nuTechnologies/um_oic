#!/bin/bash
# UM-OIC API Test Runner
# Comprehensive test suite for all API endpoints

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Test configuration
AUTH_BASE_URL="https://localhost:8443"
ADMIN_BASE_URL="https://localhost:8445"
TEST_EMAIL="admin@example.com"
TEST_PASSWORD="password123"

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Helper functions
log() {
    echo -e "${BLUE}[TEST]${NC} $1"
}

success() {
    echo -e "${GREEN}✅${NC} $1"
    ((PASSED_TESTS++))
}

fail() {
    echo -e "${RED}❌${NC} $1"
    ((FAILED_TESTS++))
}

warn() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

test_endpoint() {
    local name="$1"
    local method="$2"
    local url="$3"
    local data="$4"
    local expected_status="$5"
    local headers="$6"

    ((TOTAL_TESTS++))
    log "Testing $name..."

    local cmd="curl -k -s -w '%{http_code}' -o /tmp/test_response.json --max-time 10"

    if [ "$method" != "GET" ]; then
        cmd="$cmd -X $method"
    fi

    if [ -n "$data" ]; then
        cmd="$cmd -H 'Content-Type: application/json' -d '$data'"
    fi

    if [ -n "$headers" ]; then
        cmd="$cmd $headers"
    fi

    cmd="$cmd '$url'"

    local status_code=$(eval $cmd 2>/dev/null || echo "000")

    if [ "$status_code" = "$expected_status" ]; then
        success "$name - Status: $status_code"
        return 0
    else
        fail "$name - Expected: $expected_status, Got: $status_code"
        if [ -f /tmp/test_response.json ]; then
            echo "Response: $(cat /tmp/test_response.json)"
        fi
        return 1
    fi
}

# Start test suite
echo "🚀 UM-OIC API Test Suite"
echo "========================"
echo "Auth Service: $AUTH_BASE_URL"
echo "Admin Service: $ADMIN_BASE_URL"
echo

# Wait for services to be ready
log "Waiting for services to start..."
sleep 2

# Test 1: Auth Service Health Check
test_endpoint "Auth Health Check" "GET" "$AUTH_BASE_URL/health" "" "200"

# Test 2: OIDC Discovery
test_endpoint "OIDC Discovery" "GET" "$AUTH_BASE_URL/.well-known/openid-configuration" "" "200"

# Test 3: Login with correct credentials
ACCESS_TOKEN=""
if test_endpoint "Login Success" "POST" "$AUTH_BASE_URL/api/auth/login" '{"email":"'$TEST_EMAIL'","password":"'$TEST_PASSWORD'"}' "200"; then
    ACCESS_TOKEN=$(cat /tmp/test_response.json | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
    if [ -n "$ACCESS_TOKEN" ]; then
        success "Access token received"
    else
        fail "No access token in response"
    fi
fi

# Test 4: Login with wrong credentials
test_endpoint "Login Failure" "POST" "$AUTH_BASE_URL/api/auth/login" '{"email":"'$TEST_EMAIL'","password":"wrong"}' "200"

# Test 5: Admin Service Health Check
test_endpoint "Admin Health Check" "GET" "$ADMIN_BASE_URL/health" "" "200"

# Test 6: Admin Users API (with auth)
if [ -n "$ACCESS_TOKEN" ]; then
    test_endpoint "Admin Users List" "GET" "$ADMIN_BASE_URL/api/users" "" "200" "-H 'Authorization: Bearer $ACCESS_TOKEN'"
fi

# Test 7: Admin Claims API
if [ -n "$ACCESS_TOKEN" ]; then
    test_endpoint "Admin Claims List" "GET" "$ADMIN_BASE_URL/api/claims" "" "200" "-H 'Authorization: Bearer $ACCESS_TOKEN'"
fi

# Test 8: Admin Groups API
if [ -n "$ACCESS_TOKEN" ]; then
    test_endpoint "Admin Groups List" "GET" "$ADMIN_BASE_URL/api/groups" "" "200" "-H 'Authorization: Bearer $ACCESS_TOKEN'"
fi

# Test 9: Admin Clients API
if [ -n "$ACCESS_TOKEN" ]; then
    test_endpoint "Admin Clients List" "GET" "$ADMIN_BASE_URL/api/clients" "" "200" "-H 'Authorization: Bearer $ACCESS_TOKEN'"
fi

# Test 10: Admin Audit API
if [ -n "$ACCESS_TOKEN" ]; then
    test_endpoint "Admin Audit Query" "GET" "$ADMIN_BASE_URL/api/audit" "" "200" "-H 'Authorization: Bearer $ACCESS_TOKEN'"
fi

# Test 11: Protected endpoint without auth
test_endpoint "Unauthorized Access" "GET" "$ADMIN_BASE_URL/api/users" "" "401"

# Test 12: OAuth2 Authorize endpoint
test_endpoint "OAuth2 Authorize" "GET" "$AUTH_BASE_URL/oauth2/authorize?response_type=code&client_id=test&redirect_uri=https://example.com" "" "200"

# Test 13: Static files
test_endpoint "Static Auth Page" "GET" "$AUTH_BASE_URL/" "" "200"
test_endpoint "Static Admin Page" "GET" "$ADMIN_BASE_URL/mgmt/" "" "200"

# Test Summary
echo
echo "📊 Test Summary"
echo "==============="
echo "Total Tests: $TOTAL_TESTS"
echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed: ${RED}$FAILED_TESTS${NC}"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}💥 $FAILED_TESTS test(s) failed${NC}"
    exit 1
fi