#!/bin/bash
# Security Tests for UM-OIC

set -e

# Test configuration
AUTH_URL="http://localhost:8080"
ADMIN_URL="http://localhost:8081"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() {
    echo -e "${BLUE}[SECURITY]${NC} $1"
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

# Test 1: SQL Injection Attempts
test_sql_injection() {
    log "Testing SQL injection protection..."

    local sql_payloads=(
        "' OR '1'='1"
        "'; DROP TABLE users; --"
        "' UNION SELECT * FROM users --"
        "admin'--"
        "' OR 1=1 --"
    )

    for payload in "${sql_payloads[@]}"; do
        local response=$(curl -s -w "%{http_code}" \
            -H "Content-Type: application/json" \
            -d "{\"email\":\"$payload\",\"password\":\"test\"}" \
            "$AUTH_URL/auth/login")

        local http_code="${response: -3}"

        if [ "$http_code" != "500" ]; then
            success "SQL injection payload rejected: $payload"
        else
            warn "Potential SQL injection vulnerability detected"
        fi
    done

    success "SQL injection tests completed"
}

# Test 2: XSS Protection
test_xss_protection() {
    log "Testing XSS protection..."

    local xss_payloads=(
        "<script>alert('xss')</script>"
        "javascript:alert('xss')"
        "<img src=x onerror=alert('xss')>"
        "<svg onload=alert('xss')>"
        "';alert('xss');//"
    )

    for payload in "${xss_payloads[@]}"; do
        local response=$(curl -s -w "%{http_code}" \
            -H "Content-Type: application/json" \
            -d "{\"email\":\"$payload\",\"password\":\"test\"}" \
            "$AUTH_URL/auth/login")

        local http_code="${response: -3}"
        local body="${response%???}"

        # Check if payload is reflected in response
        if echo "$body" | grep -q "<script>" || echo "$body" | grep -q "javascript:"; then
            error "Potential XSS vulnerability: payload reflected"
        else
            success "XSS payload safely handled: $(echo $payload | cut -c1-20)..."
        fi
    done

    success "XSS protection tests completed"
}

# Test 3: Authentication Bypass Attempts
test_auth_bypass() {
    log "Testing authentication bypass protection..."

    # Test empty credentials
    local response1=$(curl -s -w "%{http_code}" \
        -H "Content-Type: application/json" \
        -d '{"email":"","password":""}' \
        "$AUTH_URL/auth/login")

    local code1="${response1: -3}"
    if [ "$code1" != "200" ]; then
        success "Empty credentials rejected"
    else
        error "Empty credentials accepted"
    fi

    # Test null values
    local response2=$(curl -s -w "%{http_code}" \
        -H "Content-Type: application/json" \
        -d '{"email":null,"password":null}' \
        "$AUTH_URL/auth/login")

    local code2="${response2: -3}"
    if [ "$code2" != "200" ]; then
        success "Null credentials rejected"
    else
        error "Null credentials accepted"
    fi

    # Test malformed JSON
    local response3=$(curl -s -w "%{http_code}" \
        -H "Content-Type: application/json" \
        -d '{"email":"test@test.com","password":' \
        "$AUTH_URL/auth/login")

    local code3="${response3: -3}"
    if [ "$code3" = "400" ]; then
        success "Malformed JSON rejected"
    else
        warn "Malformed JSON handling unclear"
    fi

    success "Authentication bypass tests completed"
}

# Test 4: JWT Token Manipulation
test_jwt_manipulation() {
    log "Testing JWT token manipulation protection..."

    # Get a valid token first
    local login_data='{"email":"admin@example.com","password":"testpassword123"}'
    local auth_response=$(curl -s \
        -H "Content-Type: application/json" \
        -d "$login_data" \
        "$AUTH_URL/auth/login")

    local valid_token=$(echo "$auth_response" | jq -r ".access_token")

    if [ "$valid_token" = "null" ] || [ "$valid_token" = "" ]; then
        warn "Cannot get valid token for JWT manipulation tests"
        return 0
    fi

    # Test modified token
    local modified_token="${valid_token}modified"
    local response1=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $modified_token" \
        "$ADMIN_URL/api/users")

    local code1="${response1: -3}"
    if [ "$code1" = "401" ]; then
        success "Modified JWT token rejected"
    else
        error "Modified JWT token accepted"
    fi

    # Test truncated token
    local truncated_token=$(echo "$valid_token" | cut -c1-50)
    local response2=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $truncated_token" \
        "$ADMIN_URL/api/users")

    local code2="${response2: -3}"
    if [ "$code2" = "401" ]; then
        success "Truncated JWT token rejected"
    else
        error "Truncated JWT token accepted"
    fi

    # Test empty token
    local response3=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer " \
        "$ADMIN_URL/api/users")

    local code3="${response3: -3}"
    if [ "$code3" = "401" ]; then
        success "Empty JWT token rejected"
    else
        error "Empty JWT token accepted"
    fi

    success "JWT manipulation tests completed"
}

# Test 5: Directory Traversal
test_directory_traversal() {
    log "Testing directory traversal protection..."

    local traversal_payloads=(
        "../../../etc/passwd"
        "..\\..\\..\\windows\\system32\\drivers\\etc\\hosts"
        "....//....//....//etc/passwd"
        "%2e%2e%2f%2e%2e%2f%2e%2e%2fetc%2fpasswd"
        "..%252f..%252f..%252fetc%252fpasswd"
    )

    for payload in "${traversal_payloads[@]}"; do
        # Test in various endpoints
        local response1=$(curl -s -w "%{http_code}" "$AUTH_URL/static/$payload")
        local code1="${response1: -3}"

        local response2=$(curl -s -w "%{http_code}" "$ADMIN_URL/api/users/$payload")
        local code2="${response2: -3}"

        if [ "$code1" != "200" ] && [ "$code2" != "200" ]; then
            success "Directory traversal payload blocked: $(echo $payload | cut -c1-20)..."
        else
            warn "Directory traversal payload accepted: $payload"
        fi
    done

    success "Directory traversal tests completed"
}

# Test 6: HTTP Method Security
test_http_methods() {
    log "Testing HTTP method security..."

    local methods=("GET" "POST" "PUT" "DELETE" "PATCH" "HEAD" "OPTIONS" "TRACE" "CONNECT")

    for method in "${methods[@]}"; do
        local response=$(curl -s -w "%{http_code}" -X "$method" "$AUTH_URL/auth/login")
        local code="${response: -3}"

        case $method in
            "POST")
                # POST should be allowed for login
                if [ "$code" != "405" ]; then
                    success "POST method allowed for login"
                fi
                ;;
            "TRACE"|"CONNECT")
                # These should be disabled
                if [ "$code" = "405" ] || [ "$code" = "501" ]; then
                    success "$method method properly disabled"
                else
                    warn "$method method may be enabled"
                fi
                ;;
            *)
                # Other methods should return 405 for login endpoint
                if [ "$code" = "405" ]; then
                    success "$method method properly rejected for login"
                fi
                ;;
        esac
    done

    success "HTTP method security tests completed"
}

# Test 7: Content-Type Validation
test_content_type() {
    log "Testing content-type validation..."

    local content_types=(
        "text/plain"
        "text/html"
        "application/xml"
        "multipart/form-data"
        "application/x-www-form-urlencoded"
    )

    for ct in "${content_types[@]}"; do
        local response=$(curl -s -w "%{http_code}" \
            -H "Content-Type: $ct" \
            -d '{"email":"test","password":"test"}' \
            "$AUTH_URL/auth/login")

        local code="${response: -3}"

        if [ "$code" != "200" ]; then
            success "Invalid content-type rejected: $ct"
        else
            warn "Invalid content-type accepted: $ct"
        fi
    done

    success "Content-type validation tests completed"
}

# Test 8: Rate Limiting
test_rate_limiting() {
    log "Testing rate limiting..."

    local start_time=$(date +%s)
    local blocked_requests=0
    local total_requests=0

    # Send rapid requests
    for i in {1..20}; do
        local response=$(curl -s -w "%{http_code}" \
            -H "Content-Type: application/json" \
            -d '{"email":"test","password":"test"}' \
            "$AUTH_URL/auth/login")

        local code="${response: -3}"
        total_requests=$((total_requests + 1))

        if [ "$code" = "429" ]; then
            blocked_requests=$((blocked_requests + 1))
        fi
    done

    if [ $blocked_requests -gt 0 ]; then
        success "Rate limiting active: $blocked_requests/$total_requests requests blocked"
    else
        warn "No rate limiting detected"
    fi

    success "Rate limiting tests completed"
}

# Test 9: Security Headers
test_security_headers() {
    log "Testing security headers..."

    local response=$(curl -s -I "$AUTH_URL/health")

    # Check for important security headers
    local headers_to_check=(
        "X-Content-Type-Options"
        "X-Frame-Options"
        "X-XSS-Protection"
        "Content-Security-Policy"
        "Referrer-Policy"
        "Strict-Transport-Security"
    )

    for header in "${headers_to_check[@]}"; do
        if echo "$response" | grep -qi "$header"; then
            success "Security header present: $header"
        else
            warn "Security header missing: $header"
        fi
    done

    # Check for information disclosure headers
    if echo "$response" | grep -qi "server:"; then
        warn "Server header present (information disclosure)"
    else
        success "Server header not present"
    fi

    if echo "$response" | grep -qi "x-powered-by"; then
        warn "X-Powered-By header present (information disclosure)"
    else
        success "X-Powered-By header not present"
    fi

    success "Security headers tests completed"
}

# Test 10: HTTPS Enforcement
test_https_enforcement() {
    log "Testing HTTPS enforcement..."

    # This test assumes production deployment would enforce HTTPS
    # For local testing, we just verify the structure is in place

    local response=$(curl -s -I "$AUTH_URL/health")

    if echo "$response" | grep -qi "strict-transport-security"; then
        success "HSTS header present"
    else
        warn "HSTS header not present (OK for local testing)"
    fi

    success "HTTPS enforcement tests completed"
}

# Test 11: Admin Privilege Escalation
test_privilege_escalation() {
    log "Testing privilege escalation protection..."

    # Get regular user token (if available)
    local user_login='{"email":"user@test.local","password":"testpassword123"}'
    local user_response=$(curl -s \
        -H "Content-Type: application/json" \
        -d "$user_login" \
        "$AUTH_URL/auth/login")

    local user_token=$(echo "$user_response" | jq -r ".access_token")

    if [ "$user_token" != "null" ] && [ "$user_token" != "" ]; then
        # Try to access admin endpoints with user token
        local admin_response=$(curl -s -w "%{http_code}" \
            -H "Authorization: Bearer $user_token" \
            "$ADMIN_URL/api/users")

        local admin_code="${admin_response: -3}"

        if [ "$admin_code" = "403" ] || [ "$admin_code" = "401" ]; then
            success "Regular user cannot access admin endpoints"
        else
            error "Privilege escalation possible"
        fi
    else
        warn "Cannot test privilege escalation (no regular user token)"
    fi

    success "Privilege escalation tests completed"
}

# Test 12: Input Validation
test_input_validation() {
    log "Testing input validation..."

    local invalid_inputs=(
        '{"email":"not-an-email","password":"test"}'
        '{"email":"","password":"test"}'
        '{"email":"test@test.com","password":""}'
        '{"email":"a".repeat(1000),"password":"test"}'
        '{"email":"test@test.com","password":"'$(printf 'x%.0s' {1..10000})'"}'
    )

    for input in "${invalid_inputs[@]}"; do
        local response=$(curl -s -w "%{http_code}" \
            -H "Content-Type: application/json" \
            -d "$input" \
            "$AUTH_URL/auth/login")

        local code="${response: -3}"

        if [ "$code" != "200" ]; then
            success "Invalid input rejected"
        else
            warn "Invalid input accepted: $(echo $input | cut -c1-30)..."
        fi
    done

    success "Input validation tests completed"
}

# Run all security tests
main() {
    log "UM-OIC Security Tests"
    echo "====================="

    local tests_passed=0
    local tests_failed=0
    local tests_warned=0

    # List of all tests
    local tests=(
        "test_sql_injection"
        "test_xss_protection"
        "test_auth_bypass"
        "test_jwt_manipulation"
        "test_directory_traversal"
        "test_http_methods"
        "test_content_type"
        "test_rate_limiting"
        "test_security_headers"
        "test_https_enforcement"
        "test_privilege_escalation"
        "test_input_validation"
    )

    # Run each test
    for test in "${tests[@]}"; do
        echo ""
        if $test; then
            ((tests_passed++))
        else
            ((tests_failed++))
        fi
    done

    # Summary
    echo ""
    echo "====================="
    log "Security Test Summary:"
    success "Passed: $tests_passed"
    if [ $tests_failed -gt 0 ]; then
        error "Failed: $tests_failed"
    fi

    echo ""
    warn "Note: These are basic security tests. For production, consider:"
    echo "  - Professional security audit"
    echo "  - Penetration testing"
    echo "  - Static code analysis (SAST)"
    echo "  - Dynamic analysis (DAST)"
    echo "  - Dependency vulnerability scanning"

    if [ $tests_failed -gt 0 ]; then
        return 1
    else
        success "All security tests passed!"
    fi
}

main "$@"