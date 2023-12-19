-- Drop unique index on users.wallet
DROP INDEX IF EXISTS idx_users_wallet;

-- Drop tables
DROP TABLE IF EXISTS transactions;
DROP TABLE IF EXISTS internal_transactions;
DROP TABLE IF EXISTS sessions_users;  -- Drop the join table
DROP TABLE IF EXISTS sessions;
DROP TABLE IF EXISTS users;
