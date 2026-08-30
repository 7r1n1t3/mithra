DROP INDEX IF EXISTS idx_security_events_created_at;

DROP INDEX IF EXISTS idx_security_events_user_id;

DROP INDEX IF EXISTS idx_external_recovery_codes_totp_secret_id;

DROP INDEX IF EXISTS idx_external_recovery_codes_user_id;

DROP INDEX IF EXISTS idx_totp_secrets_user_id;

DROP INDEX IF EXISTS idx_sessions_user_id;

DROP INDEX IF EXISTS idx_login_attempts_attempted_at;

DROP INDEX IF EXISTS idx_login_attempts_user_id;

DROP INDEX IF EXISTS idx_settings_user_id;

DROP TABLE IF EXISTS codes_changes;

DROP TABLE IF EXISTS codes_usages;

DROP TABLE IF EXISTS external_recovery_codes;

DROP TABLE IF EXISTS totp_secrets;

DROP TABLE IF EXISTS security_events;

DROP TABLE IF EXISTS sessions;

DROP TABLE IF EXISTS login_attempts;

DROP TABLE IF EXISTS settings_changes;

DROP TABLE IF EXISTS settings;

DROP TABLE IF EXISTS users_changes;

DROP TABLE IF EXISTS users;

DROP TYPE IF EXISTS password_hash_algorithm;

DROP TYPE IF EXISTS totp_algorithm;

DROP TYPE IF EXISTS event_type;

DROP TYPE IF EXISTS user_role;

DROP TYPE IF EXISTS user_status;
