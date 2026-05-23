CREATE TYPE user_status AS ENUM (
    'active',
    'locked'
);

CREATE TYPE user_role AS ENUM (
    'admin',
    'user'
);

CREATE TYPE event_type AS ENUM (
    'login_success',
    'login_failure',
    'recovery_code_used',
    'password_changed',
    'account_locked'
);

CREATE TYPE otp_algorithm AS ENUM (
    'SHA1',
    'SHA256',
    'SHA512'
);

-- User
CREATE TABLE IF NOT EXISTS users (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    username text UNIQUE NOT NULL,
    display_name text,
    email_address text UNIQUE NOT NULL,
    email_address_verified boolean DEFAULT FALSE,
    password_hash text, -- nullable for future OpenID support
    password_hash_algorithm text,
    status user_status DEFAULT 'active',
    ROLE user_role DEFAULT 'user',
    created_at timestamptz DEFAULT now(),
    updated_at timestamptz DEFAULT now(),
    CONSTRAINT p_hash_required_iff_password_provided CHECK ((password_hash IS NULL) = (password_hash_algorithm IS NULL))
);

CREATE TABLE IF NOT EXISTS settings (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id integer NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    display_language text,
    time_zone text,
    default_theme text,
    updated_at timestamptz DEFAULT now()
);

-- User security
CREATE TABLE IF NOT EXISTS login_attempts (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id integer REFERENCES users (id) ON DELETE SET NULL,
    ip_address inet,
    user_agent text,
    success boolean NOT NULL,
    attempted_at timestamptz NOT NULL,
    failure_reason text
);

CREATE TABLE IF NOT EXISTS sessions (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id integer NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    session_hash text,
    ip_address inet NOT NULL,
    user_agent text,
    created_at timestamptz DEFAULT now(),
    expires_at timestamptz DEFAULT now() + interval '1 year',
    revoked_at timestamptz
);

CREATE TABLE IF NOT EXISTS security_events (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id integer NOT NULL REFERENCES users (id) ON DELETE SET NULL,
    event_type event_type NOT NULL,
    ip_address inet,
    user_agent text,
    metadata jsonb,
    created_at timestamptz DEFAULT now()
);

-- Secrets
CREATE TABLE IF NOT EXISTS totp_secrets (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id integer NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    label text NOT NULL,
    issuer text,
    secret_key_encrypted bytea NOT NULL,
    secret_key_nonce bytea NOT NULL,
    secret_key_key_id text NOT NULL,
    otp_algorithm otp_algorithm NOT NULL DEFAULT 'SHA1',
    digits integer NOT NULL DEFAULT 6,
    period integer NOT NULL DEFAULT 30,
    enabled boolean DEFAULT TRUE,
    created_at timestamptz DEFAULT now(),
    updated_at timestamptz DEFAULT now(),
    CONSTRAINT valid_totp_digits CHECK (digits IN (6, 7, 8)),
    CONSTRAINT valid_totp_period CHECK (period > 0)
);

CREATE TABLE IF NOT EXISTS external_recovery_codes (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id integer NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    totp_secret_id integer REFERENCES totp_secrets (id) ON DELETE SET NULL,
    code_encrypted bytea NOT NULL,
    code_nonce bytea NOT NULL,
    code_key_id text NOT NULL,
    encryption_salt bytea,
    encryption_algorithm text NOT NULL,
    issuer text,
    account_name text,
    enabled boolean DEFAULT TRUE,
    used_at timestamptz,
    created_at timestamptz DEFAULT now(),
    updated_at timestamptz DEFAULT now()
);

CREATE INDEX idx_settings_user_id ON settings (user_id);

CREATE INDEX idx_login_attempts_user_id ON login_attempts (user_id);

CREATE INDEX idx_login_attempts_attempted_at ON login_attempts (attempted_at);

CREATE INDEX idx_sessions_user_id ON sessions (user_id);

CREATE INDEX idx_totp_secrets_user_id ON totp_secrets (user_id);

CREATE INDEX idx_external_recovery_codes_user_id ON external_recovery_codes (user_id);

CREATE INDEX idx_external_recovery_codes_totp_secret_id ON external_recovery_codes (totp_secret_id);

CREATE INDEX idx_security_events_user_id ON security_events (user_id);

CREATE INDEX idx_security_events_created_at ON security_events (created_at);

