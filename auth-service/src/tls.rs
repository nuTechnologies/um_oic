// Simple TLS Support für auth-service
use anyhow::{Context, Result};
use axum_server::tls_rustls::RustlsConfig;
use std::path::Path;
use tokio::fs;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
    pub domain: String,
    pub auto_generate: bool,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            cert_path: "/app/certs/cert.pem".to_string(),
            key_path: "/app/certs/key.pem".to_string(),
            domain: "localhost".to_string(),
            auto_generate: true,
        }
    }
}

/// TLS Manager für einfache Zertifikatsverwaltung
pub struct TlsManager {
    config: TlsConfig,
}

impl TlsManager {
    pub fn new(config: TlsConfig) -> Self {
        Self { config }
    }

    /// Erstelle RustlsConfig für axum-server
    pub async fn create_rustls_config(&self) -> Result<RustlsConfig> {
        // Prüfe ob Zertifikate existieren
        if !Path::new(&self.config.cert_path).exists() || !Path::new(&self.config.key_path).exists() {
            if self.config.auto_generate {
                warn!("TLS certificates not found, generating self-signed certificates");
                self.generate_self_signed_cert().await?;
            } else {
                anyhow::bail!("TLS certificates not found: {} or {}",
                    self.config.cert_path, self.config.key_path);
            }
        }

        let config = RustlsConfig::from_pem_file(&self.config.cert_path, &self.config.key_path)
            .await
            .context("Failed to load TLS certificates")?;

        info!(
            "TLS configured successfully - cert: {}, key: {}",
            self.config.cert_path, self.config.key_path
        );

        Ok(config)
    }

    /// Generiere selbstsignierte Zertifikate für Development/Testing
    async fn generate_self_signed_cert(&self) -> Result<()> {
        use rcgen::generate_simple_self_signed;

        info!("Generating self-signed certificate for: {}", self.config.domain);

        // Erstelle Subject Alternative Names
        let subject_alt_names = vec![
            self.config.domain.clone(),
            "localhost".to_string(),
            "127.0.0.1".to_string(),
        ];

        let cert = generate_simple_self_signed(subject_alt_names)
            .context("Failed to generate self-signed certificate")?;

        // Erstelle Verzeichnis falls nicht vorhanden
        if let Some(parent) = Path::new(&self.config.cert_path).parent() {
            fs::create_dir_all(parent)
                .await
                .context("Failed to create cert directory")?;
        }

        // Schreibe Zertifikat
        fs::write(&self.config.cert_path, cert.serialize_pem()?)
            .await
            .context("Failed to write certificate")?;

        // Schreibe Private Key
        fs::write(&self.config.key_path, cert.serialize_private_key_pem())
            .await
            .context("Failed to write private key")?;

        warn!("Self-signed certificate generated - not suitable for production!");
        info!("Certificate valid for: localhost, 127.0.0.1, {}", self.config.domain);

        Ok(())
    }

    /// Lade Zertifikate aus Environment oder verwende Defaults
    pub fn from_env() -> Self {
        let config = TlsConfig {
            cert_path: std::env::var("TLS_CERT_PATH")
                .unwrap_or_else(|_| "/app/certs/cert.pem".to_string()),
            key_path: std::env::var("TLS_KEY_PATH")
                .unwrap_or_else(|_| "/app/certs/key.pem".to_string()),
            domain: std::env::var("DOMAIN")
                .unwrap_or_else(|_| "localhost".to_string()),
            auto_generate: std::env::var("TLS_AUTO_GENERATE")
                .map(|v| v.parse().unwrap_or(true))
                .unwrap_or(true),
        };

        Self::new(config)
    }
}