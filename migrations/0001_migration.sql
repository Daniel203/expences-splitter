DROP TABLE IF EXISTS rooms;
DROP TABLE IF EXISTS users;

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (id, username, password) VALUES (1, 'user1', 'pass');
INSERT INTO users (id, username, password) VALUES (2, 'user2', 'pass');
INSERT INTO users (id, username, password) VALUES (3, 'user3', 'pass');

CREATE TABLE IF NOT EXISTS rooms (
    id INTEGER PRIMARY KEY,
    room_name TEXT NOT NULL,
    max_participants INTEGER NOT NULL,
    owner INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (owner) REFERENCES users (id)
);

INSERT INTO rooms (id, room_name, max_participants, owner) VALUES (1, 'Room 1', 5, 1);
INSERT INTO rooms (id, room_name, max_participants, owner) VALUES (2, 'Room 2', 3, 2);
INSERT INTO rooms (id, room_name, max_participants, owner) VALUES (3, 'Room 3', 4, 3);
