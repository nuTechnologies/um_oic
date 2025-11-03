#!/bin/bash

# Simple UM-OIC API Validation
# Direkte Tests ohne komplexe String-Escape-Probleme

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

PASSED=0
FAILED=0

success() {
    echo -e "${GREEN}✅${NC} $1"
    ((PASSED++))
}

error() {
    echo -e "${RED}❌${NC} $1"
    ((FAILED++))
}

log() {
    echo -e "${BLUE}[TEST]${NC} $1"
}

echo "🚀 Simple UM-OIC API Validation"
echo "==============================="

# Step 1: Get authentication token
log "Getting authentication token..."
TOKEN=$(curl -k -s -X POST \
    -H "Content-Type: application/json" \
    -d '{"email":"admin@example.com","password":"password123"}' \
    https://localhost:8443/api/auth/login | jq -r '.access_token')

if [ -n "$TOKEN" ] && [ "$TOKEN" != "null" ]; then
    success "Authentication successful"
else
    error "Authentication failed"
    exit 1
fi

# Step 2: Test core endpoints
log "Testing core API endpoints..."

# User info
if curl -k -s -H "Authorization: Bearer $TOKEN" https://localhost:8445/api/auth/me | jq -e '.email == "admin@example.com"' >/dev/null; then
    success "User info endpoint works"
else
    error "User info endpoint failed"
fi

# Users list (expect 4)
USER_COUNT=$(curl -k -s -H "Authorization: Bearer $TOKEN" https://localhost:8445/api/users | jq 'length')
if [ "$USER_COUNT" -eq 4 ]; then
    success "Users list: $USER_COUNT users found (expected 4)"
else
    error "Users list: $USER_COUNT users found (expected 4)"
fi

# Organizations list (expect 2)
ORG_COUNT=$(curl -k -s -H "Authorization: Bearer $TOKEN" https://localhost:8445/api/organizations | jq 'length')
if [ "$ORG_COUNT" -eq 2 ]; then
    success "Organizations list: $ORG_COUNT orgs found (expected 2)"
else
    error "Organizations list: $ORG_COUNT orgs found (expected 2)"
fi

# Clients list (expect 3)
CLIENT_COUNT=$(curl -k -s -H "Authorization: Bearer $TOKEN" https://localhost:8445/api/clients | jq 'length')
if [ "$CLIENT_COUNT" -eq 3 ]; then
    success "Clients list: $CLIENT_COUNT clients found (expected 3)"
else
    error "Clients list: $CLIENT_COUNT clients found (expected 3)"
fi

# Claims registry (expect 6 claims)
CLAIMS_COUNT=$(curl -k -s -H "Authorization: Bearer $TOKEN" https://localhost:8445/api/claims | jq '.claims | length')
if [ "$CLAIMS_COUNT" -eq 6 ]; then
    success "Claims registry: $CLAIMS_COUNT claims found (expected 6)"
else
    error "Claims registry: $CLAIMS_COUNT claims found (expected 6)"
fi

# Step 3: Test CRUD with database persistence
log "Testing CRUD operations with database persistence..."

INITIAL_USER_COUNT=$(ls ./data/users/default/ | wc -l)
log "Initial user files: $INITIAL_USER_COUNT"

# Create test user
TEST_USER_EMAIL="validation@crud.test"
CREATE_RESPONSE=$(curl -k -s -X POST \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"email":"'$TEST_USER_EMAIL'","password":"testpass123","first_name":"CRUD","last_name":"Test","org":"default","admin":[]}' \
    https://localhost:8445/api/users)

if echo "$CREATE_RESPONSE" | jq -e '.email == "'$TEST_USER_EMAIL'"' >/dev/null; then
    success "User creation API successful"

    # Check database file
    NEW_USER_COUNT=$(ls ./data/users/default/ | wc -l)
    if [ "$NEW_USER_COUNT" -gt "$INITIAL_USER_COUNT" ]; then
        success "Database: User file created (count: $INITIAL_USER_COUNT → $NEW_USER_COUNT)"
    else
        error "Database: User file not created"
    fi
else
    error "User creation API failed"
fi

# Delete test user
DELETE_RESPONSE=$(curl -k -s -w "%{http_code}" -X DELETE \
    -H "Authorization: Bearer $TOKEN" \
    https://localhost:8445/api/users/$TEST_USER_EMAIL)

if [ "${DELETE_RESPONSE: -3}" = "204" ]; then
    success "User deletion API successful"

    # Check database file deletion
    FINAL_USER_COUNT=$(ls ./data/users/default/ | wc -l)
    if [ "$FINAL_USER_COUNT" -eq "$INITIAL_USER_COUNT" ]; then
        success "Database: User file deleted (count: $NEW_USER_COUNT → $FINAL_USER_COUNT)"
    else
        error "Database: User file not deleted (count: $NEW_USER_COUNT → $FINAL_USER_COUNT)"
    fi
else
    error "User deletion API failed (HTTP: ${DELETE_RESPONSE: -3})"
fi

# Step 4: Statistics and sessions
log "Testing statistics and session endpoints..."

if curl -k -s -H "Authorization: Bearer $TOKEN" https://localhost:8445/stats/quick | jq -e '.failed_logins_today != null' >/dev/null; then
    success "Quick statistics endpoint works"
else
    error "Quick statistics endpoint failed"
fi

SESSION_COUNT=$(curl -k -s -H "Authorization: Bearer $TOKEN" https://localhost:8445/api/sessions/active | jq 'length')
if [ "$SESSION_COUNT" -ge 1 ]; then
    success "Active sessions: $SESSION_COUNT found (≥1 expected)"
else
    error "Active sessions: $SESSION_COUNT found (≥1 expected)"
fi

# Summary
echo ""
echo "📊 Test Results Summary"
echo "======================"
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo "Total: $((PASSED + FAILED))"

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}💥 $FAILED tests failed${NC}"
    exit 1
fi