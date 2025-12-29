-- Add migration script here

CREATE TYPE user_role AS ENUM ('admin', 'editor', 'author', 'subscriber');
CREATE TABLE users
(
    id             UUID PRIMARY KEY             DEFAULT gen_random_uuid(),
    username       VARCHAR(50) UNIQUE  NOT NULL,
    email          VARCHAR(100) UNIQUE NOT NULL,
    display_name   VARCHAR(100)        NOT NULL,
    bio            TEXT,
    avatar_url     TEXT,
    website        TEXT,
    password_hash  VARCHAR(255)        NOT NULL,
    role           user_role           NOT NULL DEFAULT 'subscriber',
    is_active      BOOLEAN                      DEFAULT TRUE,
    email_verified BOOLEAN                      DEFAULT FALSE,
    created_at     TIMESTAMPTZ                  DEFAULT NOW(),
    updated_at     TIMESTAMPTZ                  DEFAULT NOW(),
    last_login     TIMESTAMPTZ
);