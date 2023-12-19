-- Add up migration script here
ALTER TABLE users RENAME COLUMN device_hash TO username;
ALTER TABLE users DROP CONSTRAINT users_device_hash_key;
ALTER TABLE users ADD UNIQUE (username);
