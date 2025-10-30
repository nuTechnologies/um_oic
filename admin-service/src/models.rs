// Shared models with auth-service
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

// Include shared models from auth-service
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub status: UserStatus,
    pub verified: bool,
    pub authenticated: Option<String>, // Date of identity verification
    pub admin: Vec<String>, // Orgs user is admin for, or ["all"]
    pub org: String, // Primary organization
    pub claims: HashMap<String, serde_json::Value>, // Registry-validated claims
    pub mfa_secret: Option<String>,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub description: String,
    pub metadata: HashMap<String, serde_json::Value>,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub client_id: String,
    pub client_secret_hash: Option<String>,
    pub name: String,
    pub client_type: ClientType,
    pub redirect_uris: Vec<String>,
    pub allowed_scopes: Vec<String>,
    pub require_pkce: bool,
    pub grant_types: Vec<String>,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClientType {
    Public,
    Confidential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: String,
    pub user_id: Option<String>,
    pub org: Option<String>,
    pub event_type: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimsRegistry {
    #[serde(flatten)]
    pub claims: HashMap<String, ClaimDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimDefinition {
    #[serde(rename = "type")]
    pub claim_type: String,
    pub items: Option<serde_json::Value>,
    pub description: String,
    pub default_allowed: bool,
    pub required: Option<bool>,
    pub sensitive: Option<bool>,
    pub admin_only: Option<bool>,
}

impl User {
    pub fn new(email: String, password_hash: String, first_name: String, last_name: String, org: String) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: format!("user-{}", uuid::Uuid::new_v4().simple()),
            email,
            password_hash,
            first_name,
            last_name,
            status: UserStatus::Active,
            verified: false,
            authenticated: None,
            admin: vec![],
            org,
            claims: HashMap::new(),
            mfa_secret: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn is_admin(&self) -> bool {
        self.admin.contains(&"all".to_string()) || !self.admin.is_empty()
    }

    pub fn is_admin_for_org(&self, org: &str) -> bool {
        self.admin.contains(&"all".to_string()) || self.admin.contains(&org.to_string())
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, UserStatus::Active)
    }

    pub fn get_roles(&self) -> Vec<String> {
        self.claims.get("roles")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default()
    }
}

// Admin-specific request/response types
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub org: String,
    pub admin: Option<Vec<String>>,
    pub claims: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub status: Option<UserStatus>,
    pub org: Option<String>,
    pub admin: Option<Vec<String>>,
    pub claims: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGroupRequest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGroupRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}


#[derive(Debug, Deserialize)]
pub struct CreateClientRequest {
    pub client_id: String,
    pub name: String,
    pub client_type: ClientType,
    pub redirect_uris: Vec<String>,
    pub allowed_scopes: Vec<String>,
    pub require_pkce: Option<bool>,
    pub grant_types: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClientRequest {
    pub name: Option<String>,
    pub redirect_uris: Option<Vec<String>>,
    pub allowed_scopes: Option<Vec<String>>,
    pub require_pkce: Option<bool>,
    pub grant_types: Option<Vec<String>>,
}


#[derive(Debug, Deserialize)]
pub struct AuditQueryRequest {
    pub user_id: Option<String>,
    pub event_type: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct SystemStatus {
    pub status: String,
    pub auth_data_stale: bool,
    pub last_auth_reload: Option<OffsetDateTime>,
    pub last_data_update: OffsetDateTime,
    pub users_count: usize,
    pub groups_count: usize,
    pub clients_count: usize,
    pub organizations_count: usize,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub status: UserStatus,
    pub verified: bool,
    pub org: String,
    pub admin: Vec<String>,
    pub claims: HashMap<String, serde_json::Value>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    // Password hash is never included in responses
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            status: user.status,
            verified: user.verified,
            org: user.org,
            admin: user.admin,
            claims: user.claims,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

// JWT Claims for verification (no issuing in admin service)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub name: String,
    pub org: String, // Primary organization
    pub admin: Vec<String>, // Admin scopes
    #[serde(flatten)]
    pub user_claims: HashMap<String, serde_json::Value>, // Registry-validated claims
    pub iss: String, // issuer
    pub aud: Vec<String>, // audience
    pub exp: u64, // expiration
    pub iat: u64, // issued at
    pub jti: String, // JWT ID
}