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
) -> anyhow::Result<(TokenPair, String)> {
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
    )?;

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

pub fn decode_access_token(token: &str, jwt_secret: &str) -> anyhow::Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub async fn register<A: AuthRepository, U: UserRepository>(
    auth_repo: &A,
    user_repo: &U,
    email: &str,
    password: &str,
    display_name: &str,
    jwt_secret: &str,
) -> anyhow::Result<TokenPair> {
    // Check uniqueness
    if auth_repo.find_identity_by_email(email).await?.is_some() {
        anyhow::bail!("Email already registered");
    }

    // Hash password
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Password hash error: {e}"))?
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
) -> anyhow::Result<TokenPair> {
    let identity = auth_repo
        .find_identity_by_email(email)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid email or password"))?;

    // Verify password
    let parsed_hash = PasswordHash::new(&identity.password_hash)
        .map_err(|e| anyhow::anyhow!("Password hash error: {e}"))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| anyhow::anyhow!("Invalid email or password"))?;

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
) -> anyhow::Result<TokenPair> {
    let token_hash = hash_token(refresh_token);
    let session = auth_repo
        .find_session_by_token_hash(&token_hash)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid refresh token"))?;

    // Check expiry
    let expires_at =
        chrono::NaiveDateTime::parse_from_str(&session.expires_at, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| anyhow::anyhow!("Invalid expiry: {e}"))?;
    if Utc::now().naive_utc() > expires_at {
        auth_repo.delete_session(&session.id).await?;
        anyhow::bail!("Refresh token expired");
    }

    // Find identity to get user_id
    // We need to look up via identity_id from the session
    // Delete old session
    auth_repo.delete_session(&session.id).await?;

    // Create new session — we need to decode identity_id from the session
    // The session has identity_id, we need user_id. We'll get it from a separate lookup.
    // For now, we'll store user_id in a way we can retrieve it. Let's extend AuthRepository.
    // Actually, let's just decode the identity to get user_id.
    let identity = auth_repo
        .find_identity_by_id(&session.identity_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Identity not found"))?;

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
