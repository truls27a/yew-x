use sqlx::FromRow;

#[derive(FromRow)]
pub struct IdentityRow {
    pub id: String,
    pub user_id: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(FromRow)]
pub struct SessionRow {
    pub id: String,
    pub identity_id: String,
    pub token_hash: String,
    pub expires_at: i64,
}
