use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub display_name: String,
    pub email_address: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    pub email_address: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub failure_reason: String,
}

#[derive(Debug, Serialize)]
pub struct SignInResponse {
    pub success: bool,
    pub failure_reason: String,
}

#[derive(Debug, Serialize)]
pub struct Session {
    pub user_id: i32,
    pub session_hash: Vec<u8>,
    pub ip_address: IpAddr,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct LoginAttempt {
    pub user_id: i32,
    pub ip_address: IpAddr,
    pub user_agent: Option<String>,
    pub success: bool,
    pub attempted_at: DateTime<Utc>,
    pub failure_reason: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub email_address: String,
    pub password_hash: String,
    pub password_hash_algorithm: PasswordHashAlgorithm,
    pub user_role: UserRole,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize)]
#[sqlx(type_name = "hash_algorithm", rename_all = "snake_case")]
pub enum PasswordHashAlgorithm {
    Argon2,
    Argon2i,
    Argon2d,
    Argon2id,
    Scrypt,
    Bcrypt,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize)]
#[sqlx(type_name = "user_status", rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Locked,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
    Owner,
    Admin,
    User,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize)]
#[sqlx(type_name = "event_type", rename_all = "snake_case")]
pub enum EventType {
    LoginSuccess,
    LoginFailure,
    AccountLocked,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize)]
#[sqlx(type_name = "totp_algorithm", rename_all = "snake_case")]
pub enum TotpAlgortihm {
    SHA1,
    SHA256,
    SHA512,
}
