use super::types::{Tweet, User};

fn mock_users() -> Vec<User> {
    vec![
        User {
            id: "alice".to_string(),
            display_name: "Alice Johnson".to_string(),
            handle: "alice".to_string(),
            avatar_url: "https://i.pravatar.cc/150?u=alice".to_string(),
            bio: "Software engineer. Rust enthusiast.".to_string(),
            followers: 1200,
            following: 340,
        },
        User {
            id: "bob".to_string(),
            display_name: "Bob Smith".to_string(),
            handle: "bob".to_string(),
            avatar_url: "https://i.pravatar.cc/150?u=bob".to_string(),
            bio: "Designer & frontend dev.".to_string(),
            followers: 850,
            following: 220,
        },
        User {
            id: "carol".to_string(),
            display_name: "Carol Chen".to_string(),
            handle: "carol".to_string(),
            avatar_url: "https://i.pravatar.cc/150?u=carol".to_string(),
            bio: "Open source contributor. Coffee addict.".to_string(),
            followers: 3400,
            following: 510,
        },
    ]
}

pub fn get_all_tweets() -> Vec<Tweet> {
    let users = mock_users();
    vec![
        Tweet {
            id: "t1".to_string(),
            user: users[0].clone(),
            content: "Just shipped a new feature in Rust! The borrow checker is my best friend now. 🦀".to_string(),
            timestamp: "2h".to_string(),
            likes: 42,
            retweets: 12,
            replies: 5,
            liked: false,
            retweeted: false,
        },
        Tweet {
            id: "t2".to_string(),
            user: users[1].clone(),
            content: "Tailwind CSS makes prototyping so fast. Dark mode looks incredible with just a few utility classes.".to_string(),
            timestamp: "4h".to_string(),
            likes: 28,
            retweets: 8,
            replies: 3,
            liked: false,
            retweeted: false,
        },
        Tweet {
            id: "t3".to_string(),
            user: users[2].clone(),
            content: "WebAssembly is the future of the web. Change my mind.".to_string(),
            timestamp: "6h".to_string(),
            likes: 156,
            retweets: 45,
            replies: 23,
            liked: false,
            retweeted: false,
        },
        Tweet {
            id: "t4".to_string(),
            user: users[0].clone(),
            content: "Hot take: Yew is the best frontend framework. Type safety all the way down.".to_string(),
            timestamp: "8h".to_string(),
            likes: 89,
            retweets: 21,
            replies: 14,
            liked: false,
            retweeted: false,
        },
        Tweet {
            id: "t5".to_string(),
            user: users[2].clone(),
            content: "Just hit 1000 contributions on GitHub this year! Open source is incredibly rewarding.".to_string(),
            timestamp: "12h".to_string(),
            likes: 234,
            retweets: 56,
            replies: 18,
            liked: false,
            retweeted: false,
        },
    ]
}

pub fn get_tweets_by_user(user_id: &str) -> Vec<Tweet> {
    get_all_tweets()
        .into_iter()
        .filter(|t| t.user.id == user_id)
        .collect()
}

pub fn get_tweet_by_id(tweet_id: &str) -> Option<Tweet> {
    get_all_tweets().into_iter().find(|t| t.id == tweet_id)
}

pub fn get_user_by_id(user_id: &str) -> Option<User> {
    let mut users = mock_users();
    users.push(super::types::current_user());
    users.into_iter().find(|u| u.id == user_id)
}
