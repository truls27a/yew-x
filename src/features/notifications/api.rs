use super::types::{Notification, NotificationType};

pub fn get_notifications() -> Vec<Notification> {
    vec![
        Notification {
            id: "n1".to_string(),
            notification_type: NotificationType::Like,
            actor_name: "Alice Johnson".to_string(),
            actor_handle: "alice".to_string(),
            actor_avatar: "https://i.pravatar.cc/150?u=alice".to_string(),
            content: Some("Your tweet about Rust got a like!".to_string()),
            timestamp: "1h".to_string(),
        },
        Notification {
            id: "n2".to_string(),
            notification_type: NotificationType::Retweet,
            actor_name: "Bob Smith".to_string(),
            actor_handle: "bob".to_string(),
            actor_avatar: "https://i.pravatar.cc/150?u=bob".to_string(),
            content: Some("Your tweet was retweeted".to_string()),
            timestamp: "3h".to_string(),
        },
        Notification {
            id: "n3".to_string(),
            notification_type: NotificationType::Follow,
            actor_name: "Carol Chen".to_string(),
            actor_handle: "carol".to_string(),
            actor_avatar: "https://i.pravatar.cc/150?u=carol".to_string(),
            content: None,
            timestamp: "5h".to_string(),
        },
        Notification {
            id: "n4".to_string(),
            notification_type: NotificationType::Reply,
            actor_name: "Alice Johnson".to_string(),
            actor_handle: "alice".to_string(),
            actor_avatar: "https://i.pravatar.cc/150?u=alice".to_string(),
            content: Some("Great point! I totally agree with this.".to_string()),
            timestamp: "8h".to_string(),
        },
        Notification {
            id: "n5".to_string(),
            notification_type: NotificationType::Like,
            actor_name: "Carol Chen".to_string(),
            actor_handle: "carol".to_string(),
            actor_avatar: "https://i.pravatar.cc/150?u=carol".to_string(),
            content: Some("Your WebAssembly tweet is getting popular!".to_string()),
            timestamp: "12h".to_string(),
        },
    ]
}
