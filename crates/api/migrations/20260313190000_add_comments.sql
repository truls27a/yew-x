CREATE TABLE IF NOT EXISTS comments (
    id TEXT PRIMARY KEY,
    tweet_id TEXT NOT NULL REFERENCES tweets(id),
    user_id TEXT NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
);
