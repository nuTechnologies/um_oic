#!/bin/bash
# Auth-Service Tests

set -e

# Test configuration
AUTH_URL="http://localhost:8080"
TEST_EMAIL="admin@test.local"
TEST_PASSWORD="testpassword123"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() {
    echo -e "${BLUE}[AUTH-TEST]${NC} $1"
}

success() {
    echo -e "${GREEN}✓${NC} $1"
}

error() {
    echo -e "${RED}✗${NC} $1"
}

# Test helper functions
assert_http_code() {
    local expected=$1
    local actual=$2
    local test_name=$3

    if [ "$actual" = "$expected" ]; then
        success "$test_name: HTTP $actual"
    else
        error "$test_name: Expected HTTP $expected, got $actual"
        return 1
    fi
}

assert_json_field() {
    local json=$1
    local field=$2
    local expected=$3
    local test_name=$4

    local actual=$(echo "$json" | jq -r ".$field")
    if [ "$actual" = "$expected" ]; then
        success "$test_name: $field = $actual"
    else
        error "$test_name: Expected $field=$expected, got $actual"
        return 1
    fi
}

# Test 1: Health Check
test_health_check() {
    log "Testing health check..."

    local response=$(curl -s -w "%{http_code}" "$AUTH_URL/health")
    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "Health check"
    assert_json_field "$body" "status" "healthy" "Health status"
}

# Test 2: User Login
test_user_login() {
    log "Testing user login..."

    local login_data='{
        "email": "'$TEST_EMAIL'",
        "password": "'$TEST_PASSWORD'"
    }'

    local response=$(curl -s -w "%{http_code}" \
        -H "Content-Type: application/json" \
        -d "$login_data" \
        "$AUTH_URL/auth/login")

    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "User login"

    # Extract access token for later tests
    ACCESS_TOKEN=$(echo "$body" | jq -r ".access_token")
    if [ "$ACCESS_TOKEN" != "null" ] && [ "$ACCESS_TOKEN" != "" ]; then
        success "Login: Access token received"
    else
        error "Login: No access token received"
        return 1
    fi

    assert_json_field "$body" "token_type" "Bearer" "Token type"
}

# Test 3: Invalid Login
test_invalid_login() {
    log "Testing invalid login..."

    local login_data='{
        "email": "'$TEST_EMAIL'",
        "password": "wrongpassword"
    }'

    local response=$(curl -s -w "%{http_code}" \
        -H "Content-Type: application/json" \
        -d "$login_data" \
        "$AUTH_URL/auth/login")

    local http_code="${response: -3}"

    assert_http_code "401" "$http_code" "Invalid login"
}

# Test 4: OAuth2 Discovery
test_oauth2_discovery() {
    log "Testing OAuth2 discovery..."

    local response=$(curl -s -w "%{http_code}" "$AUTH_URL/.well-known/openid_configuration")
    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "OAuth2 discovery"
    assert_json_field "$body" "issuer" "http://localhost:8080" "Issuer"
    assert_json_field "$body" "authorization_endpoint" "http://localhost:8080/oauth2/authorize" "Authorization endpoint"
}

# Test 5: OAuth2 Authorization
test_oauth2_authorize() {
    log "Testing OAuth2 authorization..."

    local auth_url="$AUTH_URL/oauth2/authorize?response_type=code&client_id=test-client-1&redirect_uri=http://localhost:3000/callback&scope=openid+profile&state=test-state"

    local response=$(curl -s -w "%{http_code}" "$auth_url")
    local http_code="${response: -3}"

    assert_http_code "200" "$http_code" "OAuth2 authorize"
}

# Test 6: OAuth2 Token Exchange
test_oauth2_token() {
    log "Testing OAuth2 token exchange..."

    local token_data='grant_type=authorization_code&code=test-code&redirect_uri=http://localhost:3000/callback&client_id=test-client-1'

    local response=$(curl -s -w "%{http_code}" \
        -H "Content-Type: application/x-www-form-urlencoded" \
        -d "$token_data" \
        "$AUTH_URL/oauth2/token")

    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "OAuth2 token exchange"
    assert_json_field "$body" "token_type" "Bearer" "Token type"
}

# Test 7: UserInfo Endpoint
test_userinfo() {
    log "Testing UserInfo endpoint..."

    if [ -z "$ACCESS_TOKEN" ]; then
        error "No access token available for UserInfo test"
        return 1
    fi

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ACCESS_TOKEN" \
        "$AUTH_URL/oauth2/userinfo")

    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "UserInfo endpoint"
    assert_json_field "$body" "sub" "placeholder-user-id" "Subject"
}

# Test 8: User Registration (placeholder)
test_user_registration() {
    log "Testing user registration..."

    local register_data='{
        "email": "newuser@test.local",
        "password": "newpassword123",
        "first_name": "New",
        "last_name": "User"
    }'

    local response=$(curl -s -w "%{http_code}" \
        -H "Content-Type: application/json" \
        -d "$register_data" \
        "$AUTH_URL/auth/register")

    local http_code="${response: -3}"

    # Since registration is placeholder, expect 200 but check message
    assert_http_code "200" "$http_code" "User registration"
}

# Test 9: Logout
test_logout() {
    log "Testing logout..."

    if [ -z "$ACCESS_TOKEN" ]; then
        error "No access token available for logout test"
        return 1
    fi

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ACCESS_TOKEN" \
        -X POST \
        "$AUTH_URL/auth/logout")

    local http_code="${response: -3}"

    assert_http_code "200" "$http_code" "User logout"
}

# Test 10: Password Reset (placeholder)
test_password_reset() {
    log "Testing password reset..."

    local reset_data='{
        "email": "'$TEST_EMAIL'"
    }'

    local response=$(curl -s -w "%{http_code}" \
        -H "Content-Type: application/json" \
        -d "$reset_data" \
        "$AUTH_URL/auth/reset-password")

    local http_code="${response: -3}"

    assert_http_code "200" "$http_code" "Password reset"
}

# Run all auth-service tests
main() {
    log "Starting Auth-Service Tests"
    echo "============================"

    local tests_passed=0
    local tests_failed=0

    # List of all tests
    local tests=(
        "test_health_check"
        "test_user_login"
        "test_invalid_login"
        "test_oauth2_discovery"
        "test_oauth2_authorize"
        "test_oauth2_token"
        "test_userinfo"
        "test_user_registration"
        "test_logout"
        "test_password_reset"
    )

    # Run each test
    for test in "${tests[@]}"; do
        if $test; then
            ((tests_passed++))
        else
            ((tests_failed++))
        fi
        echo ""
    done

    # Summary
    echo "=========================="
    log "Auth-Service Test Summary:"
    success "Passed: $tests_passed"
    if [ $tests_failed -gt 0 ]; then
        error "Failed: $tests_failed"
        return 1
    else
        success "All tests passed!"
    fi
}

main "$@"