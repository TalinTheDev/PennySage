-- Add migration script here
CREATE TABLE IF NOT EXISTS families2 (
	id TEXT NOT NULL UNIQUE PRIMARY KEY, -- Family ID
	admin_id TEXT NOT NULL, -- Admin User
	name TEXT NOT NULL, -- Family name

	FOREIGN KEY(admin_id) REFERENCES users(id) ON UPDATE CASCADE ON DELETE RESTRICT
);

INSERT INTO families2 (id, admin_id, name)
   SELECT id, admin_id, name FROM families;

DROP TABLE families;

ALTER TABLE families2 RENAME TO families;