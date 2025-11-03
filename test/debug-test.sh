#!/bin/bash

echo "🔍 Debug Test - Finding the hang point"
echo "======================================"

set -x  # Debug mode - shows each command

echo "Step 1: Testing basic curl"
curl -k -s https://localhost:8443/health && echo "✅ Auth health OK"

echo "Step 2: Testing login request"
response=$(curl -k -s -X POST \
    -H "Content-Type: application/json" \
    -d '{"email":"admin@example.com","password":"password123"}' \
    https://localhost:8443/api/auth/login)

echo "Step 3: Response received"
echo "Response: $response"

echo "Step 4: Testing jq parsing"
success_status=$(echo "$response" | jq -r '.success // false' 2>/dev/null)
echo "Success status: $success_status"

if [ "$success_status" = "true" ]; then
    echo "Step 5: Extracting token"
    access_token=$(echo "$response" | jq -r '.access_token // empty' 2>/dev/null)
    echo "Token: ${access_token:0:50}..."

    echo "Step 6: Testing admin service"
    curl -k -s -H "Authorization: Bearer $access_token" https://localhost:8445/api/auth/me
    echo ""
    echo "✅ All steps completed successfully"
else
    echo "❌ Login failed"
fi

echo "🏁 Debug completed"