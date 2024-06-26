CREATE TABLE IF NOT EXISTS users (
	id TEXT NOT NULL UNIQUE PRIMARY KEY, -- User ID
	family_id TEXT, -- Family to which the user belongs to
	username TEXT NOT NULL UNIQUE, -- User username 
	first_name TEXT NOT NULL, -- User first name
	last_name TEXT NOT NULL, -- User last name
	email TEXT NOT NULL UNIQUE, -- User email
	password TEXT NOT NULL, -- Hashed password
	role TEXT NOT NULL, -- Admin (of a family) or general user

	FOREIGN KEY(family_id) REFERENCES families(id) ON UPDATE CASCADE ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS transactions (
	id TEXT NOT NULL UNIQUE PRIMARY KEY, -- Transaction ID
	user_id TEXT NOT NULL, -- Who completed the transaction
	family_id TEXT, -- Family to which the transaction belongs to
	description TEXT NOT NULL DEFAULT 'Transaction', -- General description of transaction
	date INTEGER NOT NULL, -- Date of transaction
	amount REAL  NOT NULL, -- Amount of transaction (can be negative/debt or positive/credit)
	merchant TEXT NOT NULL DEFAULT 'Merchant N/A', -- With whoom the transaction was conducted
	category TEXT NOT NULL DEFAULT 'Category N/A', -- General type of transaction
	payment_source TEXT NOT NULL DEFAULT 'Payment_Source N/A', -- Where the money came from
	type TEXT NOT NULL DEFAULT "debt",

	FOREIGN KEY(user_id) REFERENCES users(id) ON UPDATE CASCADE ON DELETE SET NULL
	FOREIGN KEY(family_id) REFERENCES families(id) ON UPDATE CASCADE ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS families (
	id TEXT NOT NULL UNIQUE PRIMARY KEY, -- Family ID
	admin_id TEXT NOT NULL, -- Admin User
	name TEXT NOT NULL UNIQUE, -- Family name

	FOREIGN KEY(admin_id) REFERENCES users(id) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE TABLE IF NOT EXISTS transaction_items (
	id TEXT NOT NULL UNIQUE PRIMARY KEY, -- Item ID
	transaction_id TEXT NOT NULL, -- Transaction in which item belongs
	description TEXT NOT NULL DEFAULT 'Transaction_Item', -- Description of item
	amount REAL NOT NULL, -- How much the item cost
	category TEXT NOT NULL DEFAULT 'Category N/A', -- Type of item (eg. Groceries, Electronics, etc.)

	FOREIGN KEY(transaction_id) REFERENCES transactions(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS refresh_tokens (
  id TEXT NOT NULL UNIQUE PRIMARY KEY,
  user_id TEXT NOT NULL,
  revoked INTEGER NOT NULL DEFAULT 0,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY(user_id) REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE
)