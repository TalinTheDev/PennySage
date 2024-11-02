-- Add transaction type to transactions table to allow for credit/debt transactions.
ALTER TABLE transactions ADD COLUMN type TEXT NOT NULL DEFAULT "debt";