use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

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
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
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

// Claims Registry
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

// JWT Claims (simplified)
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

// API Request/Response types
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub requires_mfa: bool,
    pub mfa_session: Option<String>,
    pub redirect_to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OAuth2AuthorizeRequest {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub state: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OAuth2TokenRequest {
    pub grant_type: String,
    pub code: Option<String>,
    pub redirect_uri: Option<String>,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub code_verifier: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OAuth2TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub org: String,
    pub verified: bool,
    #[serde(flatten)]
    pub claims: HashMap<String, serde_json::Value>,
}

impl User {
    pub fn new(email: String, password_hash: String, first_name: String, last_name: String, org: String) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: format!("user-{}", Uuid::new_v4().simple()),
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

impl AuditEvent {
    pub fn new(event_type: String, user_id: Option<String>, org: Option<String>) -> Self {
        Self {
            id: format!("evt-{}", Uuid::new_v4().simple()),
            user_id,
            org,
            event_type,
            ip_address: None,
            user_agent: None,
            metadata: HashMap::new(),
            created_at: OffsetDateTime::now_utc(),
        }
    }
}