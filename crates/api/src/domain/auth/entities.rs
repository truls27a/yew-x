#[derive(Clone, Debug, PartialEq)]
pub struct Identity {
    pub id: String,
    pub user_id: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Session {
    pub id: String,
    pub identity_id: String,
    pub token_hash: String,
    pub expires_at: i64,
}
