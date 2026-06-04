use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
    pub ip_address: IpAddr,
    pub user_agent: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub username: String,
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
    pub user_agent: String,
    pub success: bool,
    pub attempted_at: DateTime<Utc>,
    pub failure_reason: String,
}
