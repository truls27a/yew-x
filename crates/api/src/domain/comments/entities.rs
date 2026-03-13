use super::super::users::entities::User;

#[derive(Clone, Debug, PartialEq)]
pub struct Comment {
    pub id: String,
    pub user: User,
    pub content: String,
    pub created_at: i64,
}
