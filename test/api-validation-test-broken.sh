#!/bin/bash

# UM-OIC API Validation Test
# Validiert exakt die gezeigten API-Funktionen plus Database-Persistenz
# Nach RULEZ: Explizite Fehlermeldungen, keine Fallbacks

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
TEST_PASSWORD="password123"
OUTPUT_DIR="./test-results"
TIMESTAMP=$(date "+%Y%m%d_%H%M%S")
LOG_FILE="$OUTPUT_DIR/validation_test_$TIMESTAMP.log"
DATA_DIR="./data"

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Test user data
TEST_USER_EMAIL="validation@test.example.com"
TEST_USER_ID="user-validation-$(date +%s)"
TEST_ORG_ID="test-validation-org"
TEST_CLIENT_ID="test-validation-client"
TEST_CLAIM_KEY="test_validation_claim"

log() {
    echo -e "${BLUE}[VALIDATION]${NC} $1" | tee -a "$LOG_FILE"
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

# Test function with direct curl execution (no eval/string building)
validate_endpoint() {
    local method="$1"
    local endpoint="$2"
    local description="$3"
    local data="$4"
    local expected_status="$5"
    local validation_func="$6"

    ((TOTAL_TESTS++))
    log "Testing: $description"

    local response_file="/tmp/validation_response_$TOTAL_TESTS.json"
    local full_response

    # Execute request directly without eval
    if [ -f "/tmp/auth_token.txt" ]; then
        local token=$(cat /tmp/auth_token.txt)

        if [ -n "$data" ]; then
            # POST/PATCH with data
            full_response=$(curl -k -s -w '%{http_code}' -X "$method" \
                -H "Authorization: Bearer $token" \
                -H "Content-Type: application/json" \
                -d "$data" \
                "$endpoint" 2>/dev/null)
        else
            # GET/DELETE without data
            full_response=$(curl -k -s -w '%{http_code}' -X "$method" \
                -H "Authorization: Bearer $token" \
                "$endpoint" 2>/dev/null)
        fi
    else
        # No auth header
        if [ -n "$data" ]; then
            full_response=$(curl -k -s -w '%{http_code}' -X "$method" \
                -H "Content-Type: application/json" \
                -d "$data" \
                "$endpoint" 2>/dev/null)
        else
            full_response=$(curl -k -s -w '%{http_code}' -X "$method" \
                "$endpoint" 2>/dev/null)
        fi
    fi

    # Split response and status code
    local status_code="${full_response: -3}"
    local response_body="${full_response%???}"

    echo "$response_body" > "$response_file"

    # Check status code
    if [ "$status_code" = "$expected_status" ]; then
        # Run additional validation if provided
        if [ -n "$validation_func" ] && [ "$(type -t $validation_func)" = "function" ]; then
            if $validation_func "$response_body" "$response_file"; then
                success "$description - Status: $status_code ✓"
            else
                error "$description - Validation failed"
                echo "Response: $response_body" >> "$LOG_FILE"
            fi
        else
            success "$description - Status: $status_code"
        fi
    else
        error "$description - Expected: $expected_status, Got: $status_code"
        echo "Response: $response_body" >> "$LOG_FILE"
    fi
}

# Validation functions
validate_user_count() {
    local response="$1"
    local count=$(echo "$response" | jq 'length' 2>/dev/null)
    [ "$count" -eq 4 ]
}

validate_org_count() {
    local response="$1"
    local count=$(echo "$response" | jq 'length' 2>/dev/null)
    [ "$count" -eq 2 ]
}

validate_client_count() {
    local response="$1"
    local count=$(echo "$response" | jq 'length' 2>/dev/null)
    [ "$count" -eq 3 ]
}

validate_claims_registry() {
    local response="$1"
    local claims_count=$(echo "$response" | jq '.claims | length' 2>/dev/null)
    [ "$claims_count" -eq 6 ]
}

validate_quick_stats() {
    local response="$1"
    echo "$response" | jq -e '.failed_logins_today != null and .last_login_time != null and .new_users_week != null' >/dev/null 2>&1
}

validate_active_sessions() {
    local response="$1"
    local session_count=$(echo "$response" | jq 'length' 2>/dev/null)
    [ "$session_count" -ge 1 ]
}

validate_user_created() {
    local response="$1"
    echo "$response" | jq -e '.email == "'"$TEST_USER_EMAIL"'"' >/dev/null 2>&1
}

validate_claim_created() {
    local response="$1"
    echo "$response" | jq -e '.description == "Test validation claim"' >/dev/null 2>&1
}

validate_client_created() {
    local response="$1"
    echo "$response" | jq -e '.client_id == "'"$TEST_CLIENT_ID"'"' >/dev/null 2>&1
}

# Database validation functions
check_user_file_exists() {
    local user_dir="$DATA_DIR/users/default"
    local user_file="$user_dir/$TEST_USER_ID.json"

    if [ -f "$user_file" ]; then
        log "User file exists: $user_file"
        # Validate file content
        if jq -e '.email == "'"$TEST_USER_EMAIL"'"' "$user_file" >/dev/null 2>&1; then
            success "Database: User file created and contains correct data"
            return 0
        else
            error "Database: User file exists but contains incorrect data"
            return 1
        fi
    else
        error "Database: User file not created at $user_file"
        return 1
    fi
}

check_user_file_deleted() {
    local user_dir="$DATA_DIR/users/default"
    local user_file="$user_dir/$TEST_USER_ID.json"

    if [ ! -f "$user_file" ]; then
        success "Database: User file correctly deleted"
        return 0
    else
        error "Database: User file still exists after deletion: $user_file"
        return 1
    fi
}

check_client_file_exists() {
    local clients_file="$DATA_DIR/clients.json"

    if jq -e '.[] | select(.client_id == "'"$TEST_CLIENT_ID"'")' "$clients_file" >/dev/null 2>&1; then
        success "Database: Client correctly added to clients.json"
        return 0
    else
        error "Database: Client not found in clients.json"
        return 1
    fi
}

check_client_file_deleted() {
    local clients_file="$DATA_DIR/clients.json"

    if ! jq -e '.[] | select(.client_id == "'"$TEST_CLIENT_ID"'")' "$clients_file" >/dev/null 2>&1; then
        success "Database: Client correctly removed from clients.json"
        return 0
    else
        error "Database: Client still exists in clients.json after deletion"
        return 1
    fi
}

# Setup test environment
setup() {
    log "Setting up validation test environment..."
    mkdir -p "$OUTPUT_DIR"

    # Test services
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

# Authenticate with direct approach
authenticate() {
    log "Step 1: Authenticating with auth service..."

    # Direct authentication without complex string building
    local response=$(curl -k -s -X POST \
        -H "Content-Type: application/json" \
        -d '{"email":"'"$TEST_EMAIL"'","password":"'"$TEST_PASSWORD"'"}' \
        "$AUTH_SERVICE_URL/api/auth/login")

    if [ -z "$response" ]; then
        error "No response from auth service"
        exit 1
    fi

    # Simple token extraction
    local access_token=$(echo "$response" | jq -r '.access_token // empty')

    if [ -n "$access_token" ] && [ "$access_token" != "null" ] && [ "$access_token" != "empty" ]; then
        echo "$access_token" > /tmp/auth_token.txt
        success "Authentication successful (token: ${#access_token} chars)"
    else
        error "Authentication failed - no valid token"
        echo "Response: $response" >> "$LOG_FILE"
        exit 1
    fi
}

# Test core API endpoints with exact validation
test_core_apis() {
    log "Step 2: Validating core API endpoints..."

    # Authentication status
    validate_endpoint "GET" "$ADMIN_SERVICE_URL/api/auth/me" "Current user info" "" "200"

    # User management - validate exact count
    validate_endpoint "GET" "$ADMIN_SERVICE_URL/api/users" "List all users (4 expected)" "" "200" "validate_user_count"

    # Organization management - validate exact count
    validate_endpoint "GET" "$ADMIN_SERVICE_URL/api/organizations" "List organizations (2 expected)" "" "200" "validate_org_count"

    # OAuth2 clients - validate exact count
    validate_endpoint "GET" "$ADMIN_SERVICE_URL/api/clients" "List OAuth2 clients (3 expected)" "" "200" "validate_client_count"

    # Claims registry - validate structure
    validate_endpoint "GET" "$ADMIN_SERVICE_URL/api/claims" "Claims registry (6 claims expected)" "" "200" "validate_claims_registry"

    # Statistics - validate structure
    validate_endpoint "GET" "$ADMIN_SERVICE_URL/stats/quick" "Quick statistics structure" "" "200" "validate_quick_stats"

    # Active sessions - validate presence
    validate_endpoint "GET" "$ADMIN_SERVICE_URL/api/sessions/active" "Active sessions (≥1 expected)" "" "200" "validate_active_sessions"

    # Audit logs - validate accessibility
    validate_endpoint "GET" "$ADMIN_SERVICE_URL/api/audit" "Audit logs access" "" "200"
}

# Test CRUD operations with database validation
test_crud_with_database() {
    log "Step 3: Testing CRUD operations with database persistence..."

    # Create test user and validate database
    local new_user_data='{
        "email": "'"$TEST_USER_EMAIL"'",
        "password": "testpass123",
        "first_name": "Validation",
        "last_name": "Test",
        "org": "default",
        "admin": []
    }'

    validate_endpoint "POST" "$ADMIN_SERVICE_URL/api/users" "Create test user" "$new_user_data" "200" "validate_user_created"

    # Check database file creation
    ((TOTAL_TESTS++))
    if check_user_file_exists; then
        ((PASSED_TESTS++))
    else
        ((FAILED_TESTS++))
    fi

    # Create test client and validate database
    local new_client_data='{
        "client_id": "'"$TEST_CLIENT_ID"'",
        "name": "Validation Test Client",
        "client_type": "public",
        "redirect_uris": ["http://localhost:3000/callback"],
        "allowed_scopes": ["openid", "profile"]
    }'

    validate_endpoint "POST" "$ADMIN_SERVICE_URL/api/clients" "Create test client" "$new_client_data" "200" "validate_client_created"

    # Check database file update
    ((TOTAL_TESTS++))
    if check_client_file_exists; then
        ((PASSED_TESTS++))
    else
        ((FAILED_TESTS++))
    fi

    # Create test claim
    local new_claim_data='{
        "key": "'"$TEST_CLAIM_KEY"'",
        "claim_type": "string",
        "description": "Test validation claim",
        "default_allowed": false
    }'

    validate_endpoint "POST" "$ADMIN_SERVICE_URL/api/claims" "Create test claim" "$new_claim_data" "200" "validate_claim_created"

    # Test updates
    local update_user_data='{
        "first_name": "Updated Validation",
        "last_name": "Updated Test"
    }'

    validate_endpoint "PATCH" "$ADMIN_SERVICE_URL/api/users/$TEST_USER_EMAIL" "Update test user" "$update_user_data" "200"

    # Test deletions and database cleanup
    validate_endpoint "DELETE" "$ADMIN_SERVICE_URL/api/claims/$TEST_CLAIM_KEY" "Delete test claim" "" "204"
    validate_endpoint "DELETE" "$ADMIN_SERVICE_URL/api/clients/$TEST_CLIENT_ID" "Delete test client" "" "204"

    # Check database file deletion for client
    ((TOTAL_TESTS++))
    if check_client_file_deleted; then
        ((PASSED_TESTS++))
    else
        ((FAILED_TESTS++))
    fi

    validate_endpoint "DELETE" "$ADMIN_SERVICE_URL/api/users/$TEST_USER_EMAIL" "Delete test user" "" "204"

    # Check database file deletion for user
    ((TOTAL_TESTS++))
    if check_user_file_deleted; then
        ((PASSED_TESTS++))
    else
        ((FAILED_TESTS++))
    fi
}

# Test error handling
test_error_handling() {
    log "Step 4: Testing error handling..."

    validate_endpoint "GET" "$ADMIN_SERVICE_URL/api/users/nonexistent@example.com" "Get nonexistent user" "" "404"
    validate_endpoint "DELETE" "$ADMIN_SERVICE_URL/api/users/nonexistent@example.com" "Delete nonexistent user" "" "404"
    validate_endpoint "GET" "$ADMIN_SERVICE_URL/api/clients/nonexistent-client" "Get nonexistent client" "" "404"
}

# Cleanup
cleanup() {
    log "Cleaning up test environment..."
    rm -f /tmp/auth_token.txt /tmp/validation_response_*.json

    # Emergency cleanup in case deletion tests failed
    rm -f "$DATA_DIR/users/default/$TEST_USER_ID.json" 2>/dev/null || true
}

# Main execution
main() {
    echo "🚀 UM-OIC API Validation Test"
    echo "============================="
    echo "Auth Service: $AUTH_SERVICE_URL"
    echo "Admin Service: $ADMIN_SERVICE_URL"
    echo "Log File: $LOG_FILE"
    echo "Data Directory: $DATA_DIR"
    echo ""

    # Test execution
    setup
    authenticate
    test_core_apis
    test_crud_with_database
    test_error_handling
    cleanup

    # Results summary
    echo ""
    echo "📊 Validation Test Summary"
    echo "========================="
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
        echo ""
        log "All API validations passed - database persistence confirmed"
        exit 0
    fi
}

# Run main function
main "$@"