use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub notification_type: String,
    pub actor_name: String,
    pub actor_handle: String,
    pub actor_avatar: String,
    pub content: Option<String>,
    pub timestamp: String,
}
