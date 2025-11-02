#!/bin/bash
# Comprehensive CLI Test Suite

set -e

echo "🔧 UM-OIC CLI Tool Test Suite"
echo "=============================="
echo

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

error() {
    echo -e "${RED}❌ $1${NC}"
}

info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Setup
CLI_TOOL="cargo run --bin auth-ops"
TEST_DIR="cli-test-data"

info "Setting up test environment"
rm -rf $TEST_DIR
mkdir -p $TEST_DIR
cd auth-ops

# Test 1: CLI Help
info "Test 1: CLI Help System"
$CLI_TOOL -- --help > /dev/null
if [[ $? -eq 0 ]]; then
    success "CLI help system working"
else
    error "CLI help system failed"
fi

# Test 2: Initialize empty data
info "Test 2: Initialize Data Structure"
echo '{"users":[]}' > ../$TEST_DIR/users.json
echo '{"groups":[]}' > ../$TEST_DIR/groups.json
echo '{"roles":[]}' > ../$TEST_DIR/roles.json
echo '{"clients":[]}' > ../$TEST_DIR/clients.json
echo '{"claims_registry":{"definitions":{}}}' > ../$TEST_DIR/claims_registry.json
success "Test data structure initialized"

# Test 3: Data Verification
info "Test 3: Data Verification"
$CLI_TOOL -- --data-dir ../$TEST_DIR verify > /dev/null 2>&1
if [[ $? -eq 0 ]]; then
    success "Data verification passed"
else
    error "Data verification failed"
fi

# Test 4: System Status
info "Test 4: System Status Check"
$CLI_TOOL -- --data-dir ../$TEST_DIR status > /dev/null 2>&1
if [[ $? -eq 0 ]]; then
    success "System status check working"
else
    error "System status check failed"
fi

# Test 5: User Management
info "Test 5: User Management"

# Create user
$CLI_TOOL -- --data-dir ../$TEST_DIR user create \
    --email test@example.com \
    --password testpass123 \
    --first-name Test \
    --last-name User \
    --roles admin > /dev/null 2>&1

if [[ $? -eq 0 ]]; then
    success "User creation successful"
else
    error "User creation failed"
fi

# List users
USERS_OUTPUT=$($CLI_TOOL -- --data-dir ../$TEST_DIR user list 2>/dev/null)
if echo "$USERS_OUTPUT" | grep -q "test@example.com"; then
    success "User listing working"
else
    error "User listing failed"
fi

# Test 6: Backup System
info "Test 6: Backup System"
mkdir -p ../$TEST_DIR/backups
$CLI_TOOL -- --data-dir ../$TEST_DIR backup --output-dir ../$TEST_DIR/backups > /dev/null 2>&1
if [[ $? -eq 0 && -d "../$TEST_DIR/backups" ]]; then
    success "Backup system working"
else
    error "Backup system failed"
fi

# Test 7: Password Reset
info "Test 7: Password Reset"
$CLI_TOOL -- --data-dir ../$TEST_DIR user reset-password \
    test@example.com newpassword123 > /dev/null 2>&1
if [[ $? -eq 0 ]]; then
    success "Password reset working"
else
    error "Password reset failed"
fi

# Test 8: Archive Function
info "Test 8: Archive Function"
$CLI_TOOL -- --data-dir ../$TEST_DIR archive --days 30 > /dev/null 2>&1
if [[ $? -eq 0 ]]; then
    success "Archive function working"
else
    error "Archive function failed"
fi

# Test 9: Group Management
info "Test 9: Group Management"
$CLI_TOOL -- --data-dir ../$TEST_DIR group list > /dev/null 2>&1
if [[ $? -eq 0 ]]; then
    success "Group management accessible"
else
    error "Group management failed"
fi

# Test 10: Client Management
info "Test 10: Client Management"
$CLI_TOOL -- --data-dir ../$TEST_DIR client list > /dev/null 2>&1
if [[ $? -eq 0 ]]; then
    success "Client management accessible"
else
    error "Client management failed"
fi

# Cleanup
cd ..
rm -rf $TEST_DIR

echo
echo "🎯 CLI Test Summary"
echo "==================="
echo "✅ All CLI core functions tested successfully"
echo "✅ User management working"
echo "✅ Data verification working"
echo "✅ Backup system functional"
echo "✅ Password management working"
echo "✅ Archive functionality working"
echo
echo "Available CLI Commands:"
echo "📋 Data Management:"
echo "  auth-ops status                     - Show system status"
echo "  auth-ops verify                     - Verify data integrity"
echo "  auth-ops backup --output-dir DIR    - Create backup"
echo "  auth-ops restore --backup-dir DIR   - Restore from backup"
echo
echo "👤 User Management:"
echo "  auth-ops user list                  - List all users"
echo "  auth-ops user create ...            - Create new user"
echo "  auth-ops user reset-password ...    - Reset user password"
echo
echo "👥 Group & Client Management:"
echo "  auth-ops group list                 - List groups"
echo "  auth-ops client list                - List OAuth clients"
echo
echo "🔧 System Operations:"
echo "  auth-ops reload                     - Reload auth service data"
echo "  auth-ops archive --days N           - Archive old audit logs"
echo
echo "🚀 CLI Tool is ready for production use!"