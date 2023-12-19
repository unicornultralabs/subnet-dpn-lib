-- Add up migration script here
ALTER TABLE sessions
ALTER COLUMN provider_id TYPE INT8 USING provider_id::INT8,
ALTER COLUMN client_id TYPE INT8 USING client_id::INT8,
ALTER COLUMN client_id DROP NOT NULL,
ADD COLUMN status INT4 NOT NULL,
ADD CONSTRAINT fk_user
    FOREIGN KEY (provider_id)
    REFERENCES users(id),
ADD CONSTRAINT fk_client
    FOREIGN KEY (client_id)
    REFERENCES users(id);
