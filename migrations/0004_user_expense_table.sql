DROP TABLE IF EXISTS user_expense;

CREATE TABLE IF NOT EXISTS user_expense (
    user_id INTEGER NOT NULL,
    expense_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user (id)
    FOREIGN KEY (expense_id) REFERENCES expense (id)
);

INSERT INTO user_expense (user_id, expense_id) VALUES (1, 1);
