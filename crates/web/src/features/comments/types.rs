use serde::{Deserialize, Serialize};

use crate::features::users::types::User;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub user: User,
    pub content: String,
    pub timestamp: String,
}
