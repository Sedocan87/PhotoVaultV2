-- Albums
CREATE TABLE albums (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Photo-Album mapping
CREATE TABLE photo_album (
    photo_id INTEGER,
    album_id INTEGER,
    PRIMARY KEY (photo_id, album_id),
    FOREIGN KEY (photo_id) REFERENCES photos(id),
    FOREIGN KEY (album_id) REFERENCES albums(id)
);

-- Tags
CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

-- Photo-Tag mapping
CREATE TABLE photo_tag (
    photo_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (photo_id, tag_id),
    FOREIGN KEY (photo_id) REFERENCES photos(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

-- Sync journal for recovery
CREATE TABLE sync_operations (
    id TEXT PRIMARY KEY,
    operation_type TEXT NOT NULL,     -- move, delete, rename, etc.
    params JSON,
    status TEXT DEFAULT 'pending',    -- pending, completed, failed
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    error_message TEXT
);
