use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub jwt_secret: String,
    pub instance: InstanceConfig,
    pub security: SecurityConfig,
    pub features: FeaturesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceConfig {
    pub name: String,
    pub logo_url: String,
    pub primary_color: String,
    pub issuer: String,
    pub admin_client_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub password_min_length: u32,
    pub access_token_ttl: u64,
    pub refresh_token_ttl: u64,
    pub require_mfa: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub allow_registration: bool,
    pub allow_password_reset: bool,
}

impl Config {
    pub async fn load(path: &str) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            jwt_secret: "your-secret-key-change-in-production".to_string(),
            instance: InstanceConfig {
                name: "Auth Service".to_string(),
                logo_url: "/img/logo.png".to_string(),
                primary_color: "#00529F".to_string(),
                issuer: "https://auth.example.com".to_string(),
                admin_client_url: "https://localhost:8445/".to_string(),
            },
            security: SecurityConfig {
                password_min_length: 12,
                access_token_ttl: 3600,      // 1 hour
                refresh_token_ttl: 2592000,  // 30 days
                require_mfa: false,
            },
            features: FeaturesConfig {
                allow_registration: false,
                allow_password_reset: true,
            },
        }
    }
}