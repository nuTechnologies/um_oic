#!/bin/bash
# Admin-Service Tests

set -e

# Test configuration
ADMIN_URL="http://localhost:8081"
AUTH_URL="http://localhost:8080"
TEST_ADMIN_EMAIL="admin@example.com"
TEST_PASSWORD="testpassword123"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() {
    echo -e "${BLUE}[ADMIN-TEST]${NC} $1"
}

success() {
    echo -e "${GREEN}✓${NC} $1"
}

error() {
    echo -e "${RED}✗${NC} $1"
}

# Get admin token for authenticated requests
get_admin_token() {
    local login_data='{
        "email": "'$TEST_ADMIN_EMAIL'",
        "password": "'$TEST_PASSWORD'"
    }'

    local response=$(curl -s \
        -H "Content-Type: application/json" \
        -d "$login_data" \
        "$AUTH_URL/auth/login")

    ADMIN_TOKEN=$(echo "$response" | jq -r ".access_token")

    if [ "$ADMIN_TOKEN" = "null" ] || [ "$ADMIN_TOKEN" = "" ]; then
        error "Failed to get admin token"
        return 1
    fi

    success "Admin token obtained"
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
test_admin_health() {
    log "Testing admin health check..."

    local response=$(curl -s -w "%{http_code}" "$ADMIN_URL/health")
    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "Admin health check"
    assert_json_field "$body" "status" "healthy" "Health status"
}

# Test 2: System Status (requires auth)
test_system_status() {
    log "Testing system status..."

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        "$ADMIN_URL/api/system/status")

    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "System status"
    assert_json_field "$body" "status" "healthy" "System status"
}

# Test 3: List Users
test_list_users() {
    log "Testing list users..."

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        "$ADMIN_URL/api/users")

    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "List users"

    # Check if response is array
    local is_array=$(echo "$body" | jq 'type == "array"')
    if [ "$is_array" = "true" ]; then
        success "List users: Response is array"
    else
        error "List users: Response is not array"
        return 1
    fi
}

# Test 4: Get User by ID
test_get_user() {
    log "Testing get user by ID..."

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        "$ADMIN_URL/api/users/user-test001")

    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "Get user by ID"
    assert_json_field "$body" "email" "admin@example.com" "User email"
}

# Test 5: Search Users
test_search_users() {
    log "Testing search users..."

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        "$ADMIN_URL/api/users?search=admin")

    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "Search users"

    # Check if results contain admin user
    local admin_found=$(echo "$body" | jq '.[] | select(.email == "admin@example.com") | .email')
    if [ "$admin_found" = '"admin@example.com"' ]; then
        success "Search users: Admin user found"
    else
        error "Search users: Admin user not found"
        return 1
    fi
}

# Test 6: Filter Users by Status
test_filter_users() {
    log "Testing filter users by status..."

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        "$ADMIN_URL/api/users?status=active")

    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "Filter users by status"
}

# Test 7: Create User
test_create_user() {
    log "Testing create user..."

    local user_data='{
        "email": "newuser@test.local",
        "password": "newpassword123",
        "first_name": "New",
        "last_name": "User",
        "org": "test-org",
        "admin": [],
        "claims": {
            "department": "engineering"
        }
    }'

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        -H "Content-Type: application/json" \
        -d "$user_data" \
        "$ADMIN_URL/api/users")

    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "Create user"

    # Store created user ID for later tests
    CREATED_USER_ID=$(echo "$body" | jq -r ".id")
    success "User created with ID: $CREATED_USER_ID"
}

# Test 8: Update User
test_update_user() {
    log "Testing update user..."

    if [ -z "$CREATED_USER_ID" ]; then
        error "No created user ID available"
        return 1
    fi

    local update_data='{
        "first_name": "Updated",
        "claims": {
            "department": "marketing",
            "updated": "true"
        }
    }'

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        -H "Content-Type: application/json" \
        -X PUT \
        -d "$update_data" \
        "$ADMIN_URL/api/users/$CREATED_USER_ID")

    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "Update user"
    assert_json_field "$body" "first_name" "Updated" "Updated first name"
}

# Test 9: List Groups
test_list_groups() {
    log "Testing list groups..."

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        "$ADMIN_URL/api/groups")

    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "List groups"

    local is_array=$(echo "$body" | jq 'type == "array"')
    if [ "$is_array" = "true" ]; then
        success "List groups: Response is array"
    else
        error "List groups: Response is not array"
        return 1
    fi
}

# Test 10: Get Group by ID
test_get_group() {
    log "Testing get group by ID..."

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        "$ADMIN_URL/api/groups/group-admins")

    local http_code="${response: -3}"
    local body="${response%???}"

    assert_http_code "200" "$http_code" "Get group by ID"
    assert_json_field "$body" "name" "Administrators" "Group name"
}

# Test 11: List Organizations
test_list_organizations() {
    log "Testing list organizations..."

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        "$ADMIN_URL/api/organizations")

    local http_code="${response: -3}"

    assert_http_code "200" "$http_code" "List organizations"
}

# Test 12: Organization Users
test_organization_users() {
    log "Testing organization users..."

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        "$ADMIN_URL/api/organizations/test-org/users")

    local http_code="${response: -3}"

    assert_http_code "200" "$http_code" "Organization users"
}

# Test 13: Audit Log
test_audit_log() {
    log "Testing audit log..."

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        "$ADMIN_URL/api/audit")

    local http_code="${response: -3}"

    assert_http_code "200" "$http_code" "Audit log"
}

# Test 14: Password Reset (admin)
test_admin_password_reset() {
    log "Testing admin password reset..."

    if [ -z "$CREATED_USER_ID" ]; then
        error "No created user ID available"
        return 1
    fi

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        -X POST \
        "$ADMIN_URL/api/users/$CREATED_USER_ID/reset-password")

    local http_code="${response: -3}"

    assert_http_code "200" "$http_code" "Admin password reset"
}

# Test 15: Delete User
test_delete_user() {
    log "Testing delete user..."

    if [ -z "$CREATED_USER_ID" ]; then
        error "No created user ID available"
        return 1
    fi

    local response=$(curl -s -w "%{http_code}" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        -X DELETE \
        "$ADMIN_URL/api/users/$CREATED_USER_ID")

    local http_code="${response: -3}"

    assert_http_code "204" "$http_code" "Delete user"
    success "User deleted successfully"
}

# Test 16: Unauthorized Access
test_unauthorized_access() {
    log "Testing unauthorized access..."

    local response=$(curl -s -w "%{http_code}" \
        "$ADMIN_URL/api/users")

    local http_code="${response: -3}"

    assert_http_code "401" "$http_code" "Unauthorized access"
}

# Run all admin-service tests
main() {
    log "Starting Admin-Service Tests"
    echo "============================="

    # Get admin token first
    if ! get_admin_token; then
        error "Failed to get admin token, skipping authenticated tests"
        return 1
    fi

    local tests_passed=0
    local tests_failed=0

    # List of all tests
    local tests=(
        "test_admin_health"
        "test_system_status"
        "test_list_users"
        "test_get_user"
        "test_search_users"
        "test_filter_users"
        "test_create_user"
        "test_update_user"
        "test_list_groups"
        "test_get_group"
        "test_list_organizations"
        "test_organization_users"
        "test_audit_log"
        "test_admin_password_reset"
        "test_delete_user"
        "test_unauthorized_access"
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
    echo "============================="
    log "Admin-Service Test Summary:"
    success "Passed: $tests_passed"
    if [ $tests_failed -gt 0 ]; then
        error "Failed: $tests_failed"
        return 1
    else
        success "All tests passed!"
    fi
}

main "$@"