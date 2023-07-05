CREATE TABLE IF NOT EXISTS message
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    owner_id INTEGER NOT NULL,
    recipient_id INTEGER,
    creation_datestamp INTEGER NOT NULL,
    message_contents TEXT NOT NULL
);