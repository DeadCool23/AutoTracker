use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use models::{Document, Role, UserWithId};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: usize,
    pub role: Role,
    pub passport: Option<Document>,
    pub iat: usize,
}

pub fn create_jwt(user: &UserWithId) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = cfg::var("keys.jwt_key");

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        id: user.id,
        role: user.role.clone(),
        passport: user.passport.clone(),
        iat: now,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = cfg::var("keys.jwt_key");

    let mut validation = Validation::default();
    validation.validate_exp = false;
    validation.required_spec_claims.clear();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;

    Ok(token_data.claims)
}
