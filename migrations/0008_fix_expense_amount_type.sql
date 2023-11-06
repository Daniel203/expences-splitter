DROP TABLE IF EXISTS expense;

CREATE TABLE IF NOT EXISTS expense (
    id INTEGER PRIMARY KEY,
    paid_by INTEGER NOT NULL,
    amount REAL NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    room_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (paid_by) REFERENCES user (id)
    FOREIGN KEY (room_id) REFERENCES room (id)
);
