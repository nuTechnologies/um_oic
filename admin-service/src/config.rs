use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub jwt_public_key: String,
    pub instance: InstanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceConfig {
    pub name: String,
    pub issuer: String,
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
            jwt_public_key: "your-secret-key-change-in-production".to_string(),
            instance: InstanceConfig {
                name: "Admin Service".to_string(),
                issuer: "https://auth.example.com".to_string(),
            },
        }
    }
}