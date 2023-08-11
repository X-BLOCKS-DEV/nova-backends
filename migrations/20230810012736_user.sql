-- Add migration script here
CREATE TABLE users (
  id         VARCHAR(26) NOT NULL PRIMARY KEY,
  name       VARCHAR(255) NOT NULL UNIQUE,
  email      VARCHAR(255) NOT NULL,
  is_admin   BOOLEAN NOT NULL DEFAULT FALSE,
  password_hash VARCHAR(255) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
);