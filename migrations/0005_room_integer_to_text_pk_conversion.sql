DROP TABLE IF EXISTS user_expense;
DROP TABLE IF EXISTS expense;
DROP TABLE IF EXISTS room;

CREATE TABLE IF NOT EXISTS room (
    id TEXT PRIMARY KEY,
    room_name TEXT NOT NULL,
    max_participants INTEGER NOT NULL,
    owner INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (owner) REFERENCES user (id)
);

CREATE TABLE IF NOT EXISTS expense (
    id INTEGER PRIMARY KEY,
    amount INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    room_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (room_id) REFERENCES room (id)
);


CREATE TABLE IF NOT EXISTS user_expense (
    user_id INTEGER NOT NULL,
    expense_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user (id)
    FOREIGN KEY (expense_id) REFERENCES expense (id)
);
