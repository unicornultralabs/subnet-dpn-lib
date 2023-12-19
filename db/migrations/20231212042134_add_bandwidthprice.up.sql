-- Add up migration script here
CREATE TABLE user_bandwidth_price (
    user_id INT8 PRIMARY KEY,
    bandwidth_price FLOAT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);