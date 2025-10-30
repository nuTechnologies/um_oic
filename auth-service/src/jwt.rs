use anyhow::{Context, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::models::{Claims, User, ClaimsRegistry};

#[derive(Debug, Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    algorithm: Algorithm,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            algorithm: Algorithm::HS256,
        }
    }

    pub fn create_token(
        &self,
        user: &User,
        claims_registry: &ClaimsRegistry,
        audience: Vec<String>,
        issuer: &str,
        expires_in: u64,
    ) -> Result<String> {
        let now = OffsetDateTime::now_utc().unix_timestamp() as u64;
        let exp = now + expires_in;

        // Filter claims based on registry and allowance
        let allowed_claims = self.filter_allowed_claims(user, claims_registry);

        let claims = Claims {
            sub: user.id.clone(),
            email: user.email.clone(),
            name: user.full_name(),
            org: user.org.clone(),
            admin: user.admin.clone(),
            user_claims: allowed_claims,
            iss: issuer.to_string(),
            aud: audience,
            exp,
            iat: now,
            jti: Uuid::new_v4().to_string(),
        };

        let header = Header::new(self.algorithm);

        encode(&header, &claims, &self.encoding_key)
            .context("Failed to encode JWT")
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::new(self.algorithm);
        validation.validate_aud = false; // We'll validate audience manually if needed

        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)
            .context("Failed to decode JWT")?;

        Ok(token_data.claims)
    }

    pub fn refresh_token(
        &self,
        refresh_token: &str,
        user: &User,
        claims_registry: &ClaimsRegistry,
        audience: Vec<String>,
        issuer: &str,
        expires_in: u64,
    ) -> Result<String> {
        // Verify the refresh token first
        let _claims = self.verify_token(refresh_token)?;

        // Create a new access token
        self.create_token(user, claims_registry, audience, issuer, expires_in)
    }

    fn filter_allowed_claims(&self, user: &User, registry: &ClaimsRegistry) -> HashMap<String, serde_json::Value> {
        let mut allowed_claims = HashMap::new();

        for (claim_key, claim_value) in &user.claims {
            if let Some(definition) = registry.claims.get(claim_key) {
                // Check if claim is allowed based on registry rules
                let is_allowed = definition.default_allowed ||
                                user.is_admin() ||
                                (definition.admin_only.unwrap_or(false) && user.is_admin());

                if is_allowed {
                    allowed_claims.insert(claim_key.clone(), claim_value.clone());
                }
            }
        }

        allowed_claims
    }
}