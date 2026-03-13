use argon2::{
    password_hash::SaltString,
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::application::auth::ports::{HashPort, TokenPayload, TokenPort};
use crate::domain::error::AppError;

pub struct Argon2Hasher;

impl HashPort for Argon2Hasher {
    fn hash(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal {
                message: format!("Password hashing failed: {e}"),
                source: None,
            })?
            .to_string();
        Ok(hash)
    }

    fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| AppError::Internal {
            message: format!("Password hash parsing failed: {e}"),
            source: None,
        })?;
        match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(AppError::Internal {
                message: format!("Password verification failed: {e}"),
                source: None,
            }),
        }
    }
}

pub struct JwtEncoder {
    jwt_secret: String,
}

impl JwtEncoder {
    pub fn new(jwt_secret: &str) -> Self {
        Self {
            jwt_secret: jwt_secret.to_string(),
        }
    }
}

impl TokenPort for JwtEncoder {
    fn encode(&self, sub: &str, identity_id: &str, iat: usize, exp: usize) -> Result<String, AppError> {
        let payload = TokenPayload {
            sub: sub.to_string(),
            identity_id: identity_id.to_string(),
            exp,
            iat,
        };
        encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal {
            message: "JWT encoding failed".into(),
            source: Some(Box::new(e)),
        })
    }

    fn decode(&self, token: &str) -> Result<TokenPayload, AppError> {
        let token_data = decode::<TokenPayload>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::Internal {
            message: "JWT decoding failed".into(),
            source: Some(Box::new(e)),
        })?;
        Ok(token_data.claims)
    }
}
