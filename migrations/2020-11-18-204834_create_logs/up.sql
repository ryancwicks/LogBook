CREATE TABLE logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    log_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    body TEXT NOT NULL
);

/* INSERT INTO logs (body) VALUES ("demo log");
INSERT INTO logs (body) VALUES ("demo log 2"); */