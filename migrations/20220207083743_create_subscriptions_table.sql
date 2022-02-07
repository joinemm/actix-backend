-- Add migration script here
CREATE TABLE subscription (
    id INT AUTO_INCREMENT,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at DATETIME NOT NULL,
    PRIMARY KEY (id)
);