DROP TABLE IF EXISTS failed_login_attempts;
ALTER TABLE users DROP COLUMN IF EXISTS is_locked;
ALTER TABLE users DROP COLUMN IF EXISTS unlock_public_key;
