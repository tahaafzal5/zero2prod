-- Rename password column in user table
ALTER TABLE users RENAME password TO password_hash;