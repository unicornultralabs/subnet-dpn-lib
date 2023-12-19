-- Add down migration script here
ALTER TABLE users RENAME COLUMN username TO device_hash;
ALTER TABLE users ADD UNIQUE (device_hash);
ALTER TABLE users DROP CONSTRAINT users_username_key;