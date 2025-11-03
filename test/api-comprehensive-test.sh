#!/bin/bash

# UM-OIC Comprehensive API Test
# Tests auth login, dann alle admin-service endpoints
# Nach RULEZ: Explizite Fehlermeldungen, keine Fallbacks

set -e

# Colors für Output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
AUTH_SERVICE_URL="https://localhost:8443"
ADMIN_SERVICE_URL="https://localhost:8445"
TEST_EMAIL="admin@example.com"
TEST_PASSWORD="password123"
OUTPUT_DIR="./test-results"
TIMESTAMP=$(date "+%Y%m%d_%H%M%S")
LOG_FILE="$OUTPUT_DIR/api_test_$TIMESTAMP.log"

# Test statistics
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

log() {
    echo -e "${BLUE}[API-TEST]${NC} $1" | tee -a "$LOG_FILE"
}

success() {
    echo -e "${GREEN}✅${NC} $1" | tee -a "$LOG_FILE"
    ((PASSED_TESTS++))
}

error() {
    echo -e "${RED}❌${NC} $1" | tee -a "$LOG_FILE"
    ((FAILED_TESTS++))
}

warn() {
    echo -e "${YELLOW}⚠️${NC} $1" | tee -a "$LOG_FILE"
}

# Setup test environment
setup() {
    log "Setting up test environment..."
    mkdir -p "$OUTPUT_DIR"

    # Test if services are running
    if ! curl -k -s "$AUTH_SERVICE_URL/health" >/dev/null 2>&1; then
        error "Auth service not reachable at $AUTH_SERVICE_URL"
        exit 1
    fi

    if ! curl -k -s "$ADMIN_SERVICE_URL/health" >/dev/null 2>&1; then
        error "Admin service not reachable at $ADMIN_SERVICE_URL"
        exit 1
    fi

    success "Services are reachable"
}

# Cleanup function
cleanup() {
    log "Cleaning up test environment..."
    rm -f /tmp/auth_token.txt /tmp/test_*.json
}

# Test function with proper error handling
test_endpoint() {
    local method="$1"
    local endpoint="$2"
    local description="$3"
    local data="$4"
    local expected_status="$5"

    ((TOTAL_TESTS++))
    log "Testing: $description"

    local curl_cmd="curl -k -s -w '%{http_code}' -X $method"

    # Add auth header if token exists
    if [ -f "/tmp/auth_token.txt" ]; then
        local token=$(cat /tmp/auth_token.txt)
        curl_cmd="$curl_cmd -H 'Authorization: Bearer $token'"
    fi

    # Add data for POST/PATCH requests
    if [ -n "$data" ]; then
        curl_cmd="$curl_cmd -H 'Content-Type: application/json' -d '$data'"
    fi

    # Execute request and capture response
    local response_file="/tmp/test_response_$TOTAL_TESTS.json"
    local full_response=$(eval "$curl_cmd '$endpoint'" 2>/dev/null)

    # Split response and status code
    local status_code="${full_response: -3}"
    local response_body="${full_response%???}"

    echo "$response_body" > "$response_file"

    # Check status code
    if [ "$status_code" = "$expected_status" ]; then
        success "$description - Status: $status_code"

        # Pretty print JSON if valid
        if echo "$response_body" | jq empty 2>/dev/null; then
            echo "$response_body" | jq '.' >> "$LOG_FILE" 2>/dev/null || true
        else
            echo "Response: $response_body" >> "$LOG_FILE"
        fi
    else
        error "$description - Expected: $expected_status, Got: $status_code"
        echo "Response: $response_body" >> "$LOG_FILE"
    fi
}

# Step 1: Authenticate with auth service
authenticate() {
    log "Step 1: Authenticating with auth service..."

    local login_data="{\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\"}"
    local response=$(curl -k -s -X POST \
        -H "Content-Type: application/json" \
        -d "$login_data" \
        "$AUTH_SERVICE_URL/api/auth/login")

    # Check if login was successful
    local success=$(echo "$response" | jq -r '.success // false' 2>/dev/null)

    if [ "$success" = "true" ]; then
        local access_token=$(echo "$response" | jq -r '.access_token // empty' 2>/dev/null)

        if [ -n "$access_token" ] && [ "$access_token" != "null" ]; then
            echo "$access_token" > /tmp/auth_token.txt
            success "Authentication successful"
            log "Token saved for subsequent requests"
        else
            error "Authentication failed: No access token received"
            echo "Response: $response" >> "$LOG_FILE"
            exit 1
        fi
    else
        error "Authentication failed"
        echo "Response: $response" >> "$LOG_FILE"
        exit 1
    fi
}

# Step 2: Test all admin service endpoints
test_admin_endpoints() {
    log "Step 2: Testing admin service endpoints..."

    # Health check (no auth required)
    test_endpoint "GET" "$ADMIN_SERVICE_URL/health" "Health Check" "" "200"

    # Auth endpoints
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/auth/me" "Get current user info" "" "200"

    # User management endpoints
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/users" "List all users" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/users?search=admin" "Search users" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/users?status=active" "Filter users by status" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/users/user-admin" "Get specific user" "" "200"

    # Create test user
    local new_user_data='{
        "email": "test@example.com",
        "password": "testpass123",
        "first_name": "Test",
        "last_name": "User",
        "org": "default",
        "admin": []
    }'
    test_endpoint "POST" "$ADMIN_SERVICE_URL/api/users" "Create new user" "$new_user_data" "200"

    # Organization endpoints
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/organizations" "List organizations" "" "200"

    # Create test organization
    local new_org_data='{
        "id": "test-org",
        "name": "Test Organization",
        "description": "Test organization for API testing"
    }'
    test_endpoint "POST" "$ADMIN_SERVICE_URL/api/organizations" "Create organization" "$new_org_data" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/organizations/default/users" "List users in organization" "" "200"

    # Client management endpoints
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/clients" "List OAuth2 clients" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/clients/webapp-frontend" "Get specific client" "" "200"

    # Create test client
    local new_client_data='{
        "client_id": "test-client",
        "name": "Test Client",
        "client_type": "public",
        "redirect_uris": ["http://localhost:3000/callback"],
        "allowed_scopes": ["openid", "profile"]
    }'
    test_endpoint "POST" "$ADMIN_SERVICE_URL/api/clients" "Create OAuth2 client" "$new_client_data" "200"

    # Claims management endpoints
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/claims" "List claims" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/claims/registry" "Get claims registry" "" "200"

    # Create test claim
    local new_claim_data='{
        "key": "test_claim",
        "claim_type": "string",
        "description": "Test claim for API testing",
        "default_allowed": false
    }'
    test_endpoint "POST" "$ADMIN_SERVICE_URL/api/claims" "Create claim" "$new_claim_data" "200"

    # System endpoints
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/system/status" "System status" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/system/stats" "System statistics" "" "200"
    test_endpoint "POST" "$ADMIN_SERVICE_URL/api/system/reload-auth" "Reload auth service" "{}" "200"

    # Statistics endpoints
    test_endpoint "GET" "$ADMIN_SERVICE_URL/stats/users" "User statistics" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/stats/sessions" "Session statistics" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/stats/organizations" "Organization statistics" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/stats/clients" "Client statistics" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/stats/activity" "Activity data" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/stats/login-distribution" "Login distribution" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/stats/recent-activities" "Recent activities" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/stats/quick" "Quick statistics" "" "200"

    # Session management endpoints
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/sessions/active" "List active sessions" "" "200"

    # Audit log endpoints
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/audit" "Query audit logs" "" "200"
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/audit?limit=10" "Query audit logs with limit" "" "200"

    # Test error handling
    test_endpoint "GET" "$ADMIN_SERVICE_URL/api/users/nonexistent" "Get nonexistent user" "" "404"
    test_endpoint "DELETE" "$ADMIN_SERVICE_URL/api/users/nonexistent" "Delete nonexistent user" "" "404"

    # Update endpoints (if we created resources successfully)
    local update_user_data='{
        "first_name": "Updated Test",
        "last_name": "Updated User"
    }'
    test_endpoint "PATCH" "$ADMIN_SERVICE_URL/api/users/test@example.com" "Update user" "$update_user_data" "200"

    # Cleanup test data
    test_endpoint "DELETE" "$ADMIN_SERVICE_URL/api/claims/test_claim" "Delete test claim" "" "204"
    test_endpoint "DELETE" "$ADMIN_SERVICE_URL/api/clients/test-client" "Delete test client" "" "204"
    test_endpoint "DELETE" "$ADMIN_SERVICE_URL/api/organizations/test-org" "Delete test organization" "" "204"
}

# Main execution
main() {
    echo "🚀 UM-OIC Comprehensive API Test"
    echo "================================"
    echo "Auth Service: $AUTH_SERVICE_URL"
    echo "Admin Service: $ADMIN_SERVICE_URL"
    echo "Log File: $LOG_FILE"
    echo ""

    # Setup
    setup

    # Run tests
    authenticate
    test_admin_endpoints

    # Cleanup
    cleanup

    # Test summary
    echo ""
    echo "📊 Test Summary"
    echo "==============="
    echo "Total Tests: $TOTAL_TESTS"
    success "Passed: $PASSED_TESTS"
    if [ $FAILED_TESTS -gt 0 ]; then
        error "Failed: $FAILED_TESTS"
    else
        echo -e "${GREEN}Failed: $FAILED_TESTS${NC}"
    fi

    echo ""
    echo "Success Rate: $(( PASSED_TESTS * 100 / TOTAL_TESTS ))%"
    echo "Log File: $LOG_FILE"

    # Exit with appropriate code
    if [ $FAILED_TESTS -gt 0 ]; then
        exit 1
    else
        exit 0
    fi
}

# Run main function
main "$@"