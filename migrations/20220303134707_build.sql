CREATE TABLE IF NOT EXISTS userRelationship
(
    user_one_id INTEGER NOT NULL,
    user_two_id INTEGER NOT NULL,
    relationship_status TEXT NOT NULL,
    PRIMARY KEY (user_one_id, user_two_id)
);

CREATE TABLE IF NOT EXISTS friendRequests
(
    source_user_id INTEGER NOT NULL,
    target_user_id INTEGER NOT NULL,
    PRIMARY KEY (source_user_id, target_user_id)
);

CREATE TABLE IF NOT EXISTS users
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    hashed_password TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS tokens
(
    token TEXT UNIQUE NOT NULL,
    username TEXT NOT NULL UNIQUE,
    creation_time INTEGER NOT NULL
);
