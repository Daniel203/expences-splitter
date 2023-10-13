DROP TABLE IF EXISTS user_room;

CREATE TABLE IF NOT EXISTS user_room (
    user_id INTEGER,
    room_id INTEGER,
    PRIMARY KEY (user_id, room_id),
    FOREIGN KEY (user_id) REFERENCES user (id),
    FOREIGN KEY (room_id) REFERENCES room (id)
);
