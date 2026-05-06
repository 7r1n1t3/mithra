CREATE TYPE user_status AS ENUM ('active', 'locked');
CREATE TYPE user_role AS ENUM ('admin', 'user');
CREATE TYPE event_type AS ENUM (
	'login_success',
	'login_failure',
	'recovery_code_used',
	'password_changed',
	'account_locked'
);
CREATE TYPE otp_algorithm AS ENUM ('SHA1', 'SHA256', 'SHA512');

-- User
CREATE TABLE IF NOT EXISTS users (
	id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

	username  TEXT UNIQUE NOT NULL,
	display_name TEXT,

	email_address TEXT UNIQUE NOT NULL,
	email_address_verified BOOLEAN,

	password_hash TEXT NOT NULL,
	password_hash_algorithm TEXT NOT NULL,

	role user_role DEFAULT 'user',
	status user_status DEFAULT 'active',

	created_at TIMESTAMPTZ,
	updated_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS settings (
	id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,

	display_language TEXT,
	time_zone TEXT,
	default_theme TEXT,
	updated_at TIMESTAMPTZ
);

-- User security
CREATE TABLE IF NOT EXISTS login_attempts (
	id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,

	ip_address INET,
	user_agent TEXT,

	success BOOLEAN NOT NULL,
	attempted_at TIMESTAMPTZ NOT NULL,
	failure_reason TEXT
);

CREATE TABLE IF NOT EXISTS sessions (
	id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,

	session_hash TEXT,

	ip_address INET,
	user_agent TEXT,

	created_at TIMESTAMPTZ,
	expires_at TIMESTAMPTZ,
	revoked_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS security_events (
	id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE SET NULL,
	
	event_type event_type NOT NULL,
	ip_address INET,
	user_agent TEXT,
	metadata JSONB,

	created_at TIMESTAMPTZ
);

-- Secrets
CREATE TABLE IF NOT EXISTS totp_secrets (
	id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,

	label TEXT NOT NULL,
	issuer TEXT,

	secret_key_encrypted BYTEA NOT NULl,
	secret_key_nonce BYTEA NOT NULl,
	secret_key_key_id TEXT NOT NULl,

	otp_algorithm otp_algorithm NOT NULL DEFAULT 'SHA1',
	digits INTEGER NOT NULL DEFAULT 6,
	period INTEGER NOT NULL DEFAULT 30,

	enabled BOOLEAN,
	created_at TIMESTAMPTZ,
	updated_at TIMESTAMPTZ,

	CONSTRAINT valid_totp_digits CHECK (digits IN (6, 7, 8)),
    CONSTRAINT valid_totp_period CHECK (period > 0)
);

CREATE TABLE IF NOT EXISTS external_recovery_codes (
	id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	totp_secret_id INTEGER REFERENCES totp_secrets(id) ON DELETE SET NULL,

	code_encrypted BYTEA NOT NULL,
	code_nonce BYTEA NOT NULL,
	code_key_id TEXT NOT NULL,

	encryption_salt BYTEA,
	encryption_algorithm TEXT NOT NULL,

	issuer TEXT,
	account_name TEXT,

	enabled BOOLEAN,

	used_at TIMESTAMPTZ,
	created_at TIMESTAMPTZ,
	updated_at TIMESTAMPTZ
);

CREATE INDEX idx_settings_user_id
ON settings(user_id);

CREATE INDEX idx_login_attempts_user_id
ON login_attempts(user_id);

CREATE INDEX idx_login_attempts_attempted_at
ON login_attempts(attempted_at);

CREATE INDEX idx_sessions_user_id
ON sessions(user_id);

CREATE INDEX idx_totp_secrets_user_id
ON totp_secrets(user_id);

CREATE INDEX idx_external_recovery_codes_user_id
ON external_recovery_codes(user_id);

CREATE INDEX idx_external_recovery_codes_totp_secret_id
ON external_recovery_codes(totp_secret_id);

CREATE INDEX idx_security_events_user_id
ON security_events(user_id);

CREATE INDEX idx_security_events_created_at
ON security_events(created_at);
