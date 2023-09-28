DROP TABLE IF EXISTS room;

CREATE TABLE IF NOT EXISTS room (
    id INTEGER PRIMARY KEY,
    room_name TEXT NOT NULL,
    max_participants INTEGER NOT NULL,
    owner INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (owner) REFERENCES user (id)
);

INSERT INTO room (id, room_name, max_participants, owner) VALUES (1, 'Session 1', 5, 1);
