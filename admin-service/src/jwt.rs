use anyhow::{Context, Result};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::models::Claims;

#[derive(Clone)]
pub struct JwtVerifier {
    decoding_key: DecodingKey,
    algorithm: Algorithm,
}

impl JwtVerifier {
    pub fn new(secret: &str) -> Self {
        Self {
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            algorithm: Algorithm::HS256,
        }
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::new(self.algorithm);
        validation.validate_aud = false; // We'll validate audience manually if needed

        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)
            .context("Failed to decode JWT")?;

        Ok(token_data.claims)
    }

    pub fn has_admin_role(&self, claims: &Claims) -> bool {
        claims.admin.contains(&"all".to_string()) ||
        !claims.admin.is_empty()
    }

    pub fn has_write_permission(&self, claims: &Claims) -> bool {
        claims.admin.contains(&"all".to_string())
    }
}