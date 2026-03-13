use argon2::{
    password_hash::SaltString,
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::ports::AuthRepository;
use crate::application::users::ports::UserRepository;
use crate::domain::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub identity_id: String,
    pub exp: usize,
    pub iat: usize,
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn generate_token_pair(
    user_id: &str,
    identity_id: &str,
    jwt_secret: &str,
) -> Result<(TokenPair, String), AppError> {
    let now = Utc::now().timestamp() as usize;

    let access_claims = Claims {
        sub: user_id.to_string(),
        identity_id: identity_id.to_string(),
        exp: now + 15 * 60, // 15 minutes
        iat: now,
    };

    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal {
        message: "JWT encoding failed".into(),
        source: Some(Box::new(e)),
    })?;

    let refresh_token = uuid::Uuid::new_v4().to_string();
    let refresh_hash = hash_token(&refresh_token);

    Ok((
        TokenPair {
            access_token,
            refresh_token,
        },
        refresh_hash,
    ))
}

pub fn decode_access_token(token: &str, jwt_secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::Internal {
        message: "JWT decoding failed".into(),
        source: Some(Box::new(e)),
    })?;
    Ok(token_data.claims)
}

pub async fn register<A: AuthRepository, U: UserRepository>(
    auth_repo: &A,
    user_repo: &U,
    email: &str,
    password: &str,
    display_name: &str,
    jwt_secret: &str,
) -> Result<TokenPair, AppError> {
    // Check uniqueness
    if auth_repo.find_identity_by_email(email).await?.is_some() {
        return Err(AppError::Conflict {
            resource_type: "Identity",
            reason: "Email already registered",
        });
    }

    // Hash password
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal {
            message: format!("Password hashing failed: {e}"),
            source: None,
        })?
        .to_string();

    // Create user
    let user_id = uuid::Uuid::new_v4().to_string();
    let handle = email.split('@').next().unwrap_or("user");
    let avatar_url = format!("https://i.pravatar.cc/150?u={}", handle);
    user_repo
        .create(&user_id, display_name, handle, &avatar_url)
        .await?;

    // Create identity
    let identity_id = uuid::Uuid::new_v4().to_string();
    auth_repo
        .create_identity(&identity_id, &user_id, email, &password_hash)
        .await?;

    // Create session
    let (token_pair, refresh_hash) = generate_token_pair(&user_id, &identity_id, jwt_secret)?;
    let session_id = uuid::Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + chrono::Duration::days(7))
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    auth_repo
        .create_session(&session_id, &identity_id, &refresh_hash, &expires_at)
        .await?;

    Ok(token_pair)
}

pub async fn login<A: AuthRepository>(
    auth_repo: &A,
    email: &str,
    password: &str,
    jwt_secret: &str,
) -> Result<TokenPair, AppError> {
    let identity = auth_repo
        .find_identity_by_email(email)
        .await?
        .ok_or(AppError::Unauthorized {
            reason: "Invalid credentials",
        })?;

    // Verify password
    let parsed_hash =
        PasswordHash::new(&identity.password_hash).map_err(|e| AppError::Internal {
            message: format!("Password hash parsing failed: {e}"),
            source: None,
        })?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized {
            reason: "Invalid credentials",
        })?;

    // Create session
    let (token_pair, refresh_hash) =
        generate_token_pair(&identity.user_id, &identity.id, jwt_secret)?;
    let session_id = uuid::Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + chrono::Duration::days(7))
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    auth_repo
        .create_session(&session_id, &identity.id, &refresh_hash, &expires_at)
        .await?;

    Ok(token_pair)
}

pub async fn refresh<A: AuthRepository>(
    auth_repo: &A,
    refresh_token: &str,
    jwt_secret: &str,
) -> Result<TokenPair, AppError> {
    let token_hash = hash_token(refresh_token);
    let session = auth_repo
        .find_session_by_token_hash(&token_hash)
        .await?
        .ok_or(AppError::Unauthorized {
            reason: "Invalid refresh token",
        })?;

    // Check expiry
    let expires_at =
        chrono::NaiveDateTime::parse_from_str(&session.expires_at, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| AppError::Internal {
                message: "Invalid session expiry format".into(),
                source: Some(Box::new(e)),
            })?;
    if Utc::now().naive_utc() > expires_at {
        auth_repo.delete_session(&session.id).await?;
        return Err(AppError::Unauthorized {
            reason: "Refresh token expired",
        });
    }

    // Delete old session
    auth_repo.delete_session(&session.id).await?;

    let identity = auth_repo
        .find_identity_by_id(&session.identity_id)
        .await?
        .ok_or(AppError::NotFound {
            resource_type: "Identity",
            field: "id",
            value: session.identity_id.clone(),
        })?;

    let (token_pair, refresh_hash) =
        generate_token_pair(&identity.user_id, &identity.id, jwt_secret)?;
    let session_id = uuid::Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + chrono::Duration::days(7))
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    auth_repo
        .create_session(&session_id, &identity.id, &refresh_hash, &expires_at)
        .await?;

    Ok(token_pair)
}
