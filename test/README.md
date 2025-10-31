# UM-OIC Test Suite

Umfassende Test-Suite für das User Management OpenID Connect (UM-OIC) System.

## Test-Struktur

### Unit Tests
- `auth-service/` - Tests für den Authentifizierungs-Service
- `admin-service/` - Tests für den Admin-Service

### Integration Tests
- `integration/` - Service-übergreifende Tests
- `data/` - Test-Daten und Fixtures

## Test-Kategorien

### 1. Authentication Tests
- User Login/Logout
- JWT Token Erstellung und Validierung
- Password Hashing/Verification
- MFA (Placeholder)

### 2. Authorization Tests
- Admin Role Verification
- Organization-based Access Control
- Claims Validation

### 3. User Management Tests
- User CRUD Operations
- User Search und Filtering
- Organization Assignment

### 4. OAuth2/OIDC Tests
- Authorization Flow
- Token Exchange
- UserInfo Endpoint
- Discovery Endpoint

### 5. API Integration Tests
- Admin-Service ↔ Auth-Service Communication
- Error Handling
- Health Checks

## Test ausführen

```bash
# Alle Tests
./test/run-tests.sh

# Einzelne Services
./test/run-tests.sh auth-service
./test/run-tests.sh admin-service

# Integration Tests
./test/run-tests.sh integration
```

## Test-Daten

Test-Daten werden automatisch erstellt und nach jedem Test bereinigt.