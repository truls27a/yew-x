INSERT INTO users (id, display_name, handle, avatar_url, bio, followers, following) VALUES
    ('alice', 'Alice Johnson', 'alice', 'https://i.pravatar.cc/150?u=alice', 'Software engineer. Rust enthusiast.', 1200, 340),
    ('bob', 'Bob Smith', 'bob', 'https://i.pravatar.cc/150?u=bob', 'Designer & frontend dev.', 850, 220),
    ('carol', 'Carol Chen', 'carol', 'https://i.pravatar.cc/150?u=carol', 'Open source contributor. Coffee addict.', 3400, 510);

INSERT INTO tweets (id, user_id, content, created_at) VALUES
    ('t1', 'alice', 'Just shipped a new feature in Rust! The borrow checker is my best friend now.', unixepoch('now', '-2 hours')),
    ('t2', 'bob', 'Tailwind CSS makes prototyping so fast. Dark mode looks incredible with just a few utility classes.', unixepoch('now', '-4 hours')),
    ('t3', 'carol', 'WebAssembly is the future of the web. Change my mind.', unixepoch('now', '-6 hours')),
    ('t4', 'alice', 'Hot take: Yew is the best frontend framework. Type safety all the way down.', unixepoch('now', '-8 hours')),
    ('t5', 'carol', 'Just hit 1000 contributions on GitHub this year! Open source is incredibly rewarding.', unixepoch('now', '-12 hours'));
