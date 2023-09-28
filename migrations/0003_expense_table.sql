DROP TABLE IF EXISTS expense;

CREATE TABLE IF NOT EXISTS expense (
    id INTEGER PRIMARY KEY,
    amount INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    room_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (room_id) REFERENCES room (id)
);

INSERT INTO expense (id, amount, title, description, room_id) VALUES (1, 100, 'Expense 1', 'Description 1', 1);

