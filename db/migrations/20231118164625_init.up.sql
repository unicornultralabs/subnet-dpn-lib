-- Create users table
CREATE TABLE users (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    fingerprint VARCHAR(512),
    pincode VARCHAR,
    device_hash VARCHAR UNIQUE,
    deposit_addr VARCHAR(42) NOT NULL UNIQUE,
    withdrawal_addr VARCHAR(42),
    balance BIGINT NOT NULL DEFAULT 0,
    created_at BIGINT NOT NULL,
    last_login BIGINT NOT NULL
);

CREATE TABLE referrals(
    user_id INT8 PRIMARY KEY,
    referral_code VARCHAR(16) UNIQUE,
    created_at BIGINT NOT NULL,
    referred_by INT8,
    referred_at BIGINT,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (referred_by) REFERENCES users (id)
);

-- Create tier table
CREATE TABLE user_tiers (
    user_id INT8 PRIMARY KEY,
    tier INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

-- Create sessions table
CREATE TABLE sessions (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    provider_id VARCHAR(42) NOT NULL,
    client_id VARCHAR(42) NOT NULL,
    rate_per_second BIGINT,
    rate_per_kb BIGINT,
    handshake_at BIGINT,
    end_at BIGINT,
    duration BIGINT,
    bandwidth_usage BIGINT,
    duration_fee BIGINT,
    bandwidth_fee BIGINT,
    total_fee BIGINT
);

-- Create sessions_users join table
CREATE TABLE sessions_users (
    session_id INT8 NOT NULL,
    user_id INT8 NOT NULL,
    PRIMARY KEY (session_id, user_id),
    FOREIGN KEY (session_id) REFERENCES sessions (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

-- Create internal_transactions table
CREATE TABLE internal_transactions (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    session_id INT8 NOT NULL,
    from_user_id INT8 NOT NULL,
    to_user_id INT8 NOT NULL,
    amount BIGINT NOT NULL,
    tx_type INT NOT NULL,
    tx_status INT NOT NULL,
    rewarded BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (session_id) REFERENCES sessions (id),
    FOREIGN KEY (from_user_id) REFERENCES users (id),
    FOREIGN KEY (to_user_id) REFERENCES users (id)
);

-- Create transactions table
CREATE TABLE transactions (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    user_id INT8,
    from_addr VARCHAR(42) NOT NULL,
    to_addr VARCHAR(42) NOT NULL,
    amount BIGINT NOT NULL,
    tx_hash VARCHAR(66) UNIQUE,
    tx_type INT NOT NULL,
    tx_status INT NOT NULL,
    attempts SMALLINT NOT NULL DEFAULT 0,
    no_retry BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create unique index on users.deposit_addr
CREATE UNIQUE INDEX idx_users_deposit_addr ON users (deposit_addr);