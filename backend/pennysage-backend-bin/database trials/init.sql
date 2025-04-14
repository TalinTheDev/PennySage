CREATE TABLE IF NOT EXISTS users(
  user_id TEXT NOT NULL UNIQUE PRIMARY KEY, -- User ID
  family_id TEXT, -- User's family

  username TEXT NOT NULL UNIQUE, -- User's username
  email TEXT NOT NULL UNIQUE, -- User's email address
  password TEXT NOT NULL UNIQUE, -- User's password
  role INTEGER NOT NULL DEFAULT 0, -- User's role where 0 means regular user, 1 means family admin, and 2 means dev
  
  first_name TEXT NOT NULL, -- User's first name
  last_name TEXT NOT NULL -- User's last name
) WITHOUT ROWID;

CREATE INDEX IDX_USER_USERNAME on users(username);

-- CREATE TABLE IF NOT EXISTS families();

-- CREATE TABLE IF NOT EXISTS refresh_tokens();

-- CREATE TABLE IF NOT EXISTS transactions();

-- CREATE TABLE IF NOT EXISTS transactionItems();