# 🔧 Admin-Service API Extensions

## 📋 Overview

Diese Dokumentation beschreibt die erforderlichen API-Erweiterungen für das Admin-Service, um eine vollständige Vue.js Admin-App zu unterstützen. Die aktuellen APIs sind funktional, aber für eine moderne Admin-Oberfläche sind zusätzliche Endpoints erforderlich.

## ✅ Bestehende API-Abdeckung

### User Management (Vollständig)
```rust
GET    /api/users              ✅ List with filtering
POST   /api/users              ✅ Create user
GET    /api/users/:id          ✅ Get user details
PATCH  /api/users/:id          ✅ Update user
DELETE /api/users/:id          ✅ Delete user
POST   /api/users/:id/reset-password ✅ Password reset
```

### Organizations (Basis vorhanden)
```rust
GET    /api/organizations      ✅ List organizations
GET    /api/organizations/:org/users ✅ Get org users
```

### Groups (Vollständig)
```rust
GET    /api/groups             ✅ List groups
POST   /api/groups             ✅ Create group
GET    /api/groups/:id         ✅ Get group
PATCH  /api/groups/:id         ✅ Update group
DELETE /api/groups/:id         ✅ Delete group
```

### OAuth2 Clients (Vollständig)
```rust
GET    /api/clients            ✅ List clients
POST   /api/clients            ✅ Create client
GET    /api/clients/:id        ✅ Get client
PATCH  /api/clients/:id        ✅ Update client
DELETE /api/clients/:id        ✅ Delete client
POST   /api/clients/:id/rotate-secret ✅ Rotate secret
```

### System & Audit (Basis vorhanden)
```rust
GET    /api/system/status      ✅ System status
POST   /api/system/reload-auth ✅ Trigger auth reload
GET    /api/audit              ✅ Basic audit query
```

## ❌ Fehlende API-Endpoints

### 1. Claims Registry Management

```rust
// handlers/claims.rs - NEW FILE REQUIRED
GET    /api/claims/registry
PATCH  /api/claims/registry
GET    /api/claims/definitions
```

**Implementation needed:**
```rust
pub async fn get_claims_registry(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ClaimsRegistry>, StatusCode> {
    let storage_guard = storage.read().await;
    let registry = storage_guard.get_claims_registry();
    Ok(Json(registry.clone()))
}

pub async fn update_claims_registry(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(update_request): Json<UpdateClaimsRegistryRequest>,
) -> Result<Json<ClaimsRegistry>, StatusCode> {
    // Validate admin permissions
    // Update claims.conf file
    // Trigger auth-service reload
    todo!()
}
```

### 2. Enhanced User Management

```rust
// Extend handlers/users.rs
GET    /api/users/:id/sessions     // Active user sessions
DELETE /api/users/:id/sessions     // Force logout user
POST   /api/users/:id/verify       // Verify email address
POST   /api/users/:id/mfa/enable   // Enable MFA
POST   /api/users/:id/mfa/disable  // Disable MFA
GET    /api/users/:id/mfa/status   // MFA status
```

### 3. Bulk Operations

```rust
// handlers/bulk.rs - NEW FILE REQUIRED
POST   /api/users/bulk-create      // Bulk user creation
POST   /api/users/bulk-update      // Bulk user updates
POST   /api/users/bulk-delete      // Bulk user deletion
POST   /api/users/import           // CSV import
POST   /api/users/import/preview   // Preview import
```

### 4. Statistics & Analytics

```rust
// handlers/stats.rs - NEW FILE REQUIRED
GET    /api/stats/users            // User statistics
GET    /api/stats/organizations    // Organization statistics
GET    /api/stats/activity         // Login/activity analytics
GET    /api/stats/audit            // Audit statistics
```

### 5. Advanced Search

```rust
// Extend existing handlers
POST   /api/search/users           // Advanced user search
POST   /api/search/audit           // Advanced audit search
```

### 6. System Configuration

```rust
// handlers/config.rs - NEW FILE REQUIRED
GET    /api/config                 // Get system configuration
PATCH  /api/config                 // Update configuration
```

## 🚧 Implementation Priority

### Phase 1: Essential Extensions (High Priority)

1. **Claims Registry Management**
   - Required for proper claims editing in UI
   - File: `handlers/claims.rs`
   - Routes: `/api/claims/*`

2. **Enhanced Statistics**
   - Dashboard requires user/org statistics
   - File: `handlers/stats.rs`
   - Routes: `/api/stats/*`

3. **Bulk Operations**
   - Essential for admin productivity
   - File: `handlers/bulk.rs`
   - Routes: `/api/users/bulk-*`

### Phase 2: Advanced Features (Medium Priority)

4. **Session Management**
   - Extend `handlers/users.rs`
   - Add session tracking to storage layer

5. **Advanced Search**
   - Extend existing list endpoints with POST variants
   - Support complex query structures

6. **System Configuration**
   - Runtime config updates
   - File: `handlers/config.rs`

### Phase 3: Optional Enhancements (Low Priority)

7. **Import/Export**
   - CSV import/export functionality
   - Batch processing capabilities

8. **Real-time Updates**
   - WebSocket support for live updates
   - Server-sent events for notifications

## 📝 Required Model Extensions

### New Request/Response Types

```rust
// models.rs additions

#[derive(Debug, Deserialize)]
pub struct UpdateClaimsRegistryRequest {
    pub claims: HashMap<String, ClaimDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct BulkUserRequest {
    pub operation: BulkOperation,
    pub user_ids: Vec<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct UserStatistics {
    pub total_users: usize,
    pub active_users: usize,
    pub users_by_org: HashMap<String, usize>,
    pub users_by_status: HashMap<String, usize>,
}

#[derive(Debug, Serialize)]
pub struct SystemConfiguration {
    pub instance: InstanceConfig,
    pub security: SecurityConfig,
    pub features: FeatureConfig,
}
```

### Storage Layer Extensions

```rust
// storage.rs additions
impl AdminStorage {
    pub async fn get_user_sessions(&self, user_id: &str) -> Vec<UserSession> {
        // Implementation needed
        todo!()
    }

    pub async fn get_user_statistics(&self) -> UserStatistics {
        let total_users = self.users_count();
        let users_by_org = self.get_users_by_organization();
        // ... calculate statistics
        todo!()
    }

    pub async fn bulk_update_users(
        &mut self,
        user_ids: &[String],
        update_data: &UpdateUserRequest
    ) -> Result<BulkOperationResult> {
        // Implementation needed
        todo!()
    }
}
```

## 🔧 Router Extensions

### Updated main.rs router configuration

```rust
// main.rs - Extended router
let app = Router::new()
    // ... existing routes ...

    // Claims Registry
    .route("/api/claims/registry", get(handlers::claims::get_registry).patch(handlers::claims::update_registry))
    .route("/api/claims/definitions", get(handlers::claims::get_definitions))

    // Statistics
    .route("/api/stats/users", get(handlers::stats::user_statistics))
    .route("/api/stats/organizations", get(handlers::stats::org_statistics))
    .route("/api/stats/activity", get(handlers::stats::activity_statistics))

    // Bulk Operations
    .route("/api/users/bulk-create", post(handlers::bulk::bulk_create_users))
    .route("/api/users/bulk-update", post(handlers::bulk::bulk_update_users))
    .route("/api/users/bulk-delete", post(handlers::bulk::bulk_delete_users))
    .route("/api/users/import", post(handlers::bulk::import_users))
    .route("/api/users/import/preview", post(handlers::bulk::preview_import))

    // Enhanced User Management
    .route("/api/users/:id/sessions", get(handlers::users::get_sessions).delete(handlers::users::delete_sessions))
    .route("/api/users/:id/verify", post(handlers::users::verify_email))
    .route("/api/users/:id/mfa/enable", post(handlers::users::enable_mfa))
    .route("/api/users/:id/mfa/disable", post(handlers::users::disable_mfa))
    .route("/api/users/:id/mfa/status", get(handlers::users::mfa_status))

    // Advanced Search
    .route("/api/search/users", post(handlers::search::search_users))
    .route("/api/search/audit", post(handlers::search::search_audit))

    // System Configuration
    .route("/api/config", get(handlers::config::get_config).patch(handlers::config::update_config))

    // ... rest of router configuration
```

## 📊 Response Format Standardization

### Consistent API Response Format

```rust
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub pages: u32,
}
```

## 🎯 Implementation Timeline

**Week 1-2: Claims Registry & Statistics**
- Implement claims registry endpoints
- Add user/org statistics endpoints
- Update Vue.js stores to consume new APIs

**Week 3-4: Bulk Operations**
- Implement bulk user operations
- Add CSV import/export functionality
- Create bulk operation UI components

**Week 5-6: Session Management & Advanced Search**
- Add session management endpoints
- Implement advanced search functionality
- Create corresponding UI components

**Week 7-8: System Configuration & Polish**
- Add system configuration endpoints
- Implement real-time updates
- Polish and testing

Diese Erweiterungen würden das Admin-Service zu einer vollständigen Enterprise-tauglichen Lösung machen, die alle Anforderungen einer modernen Admin-Oberfläche erfüllt.