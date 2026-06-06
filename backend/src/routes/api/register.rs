use actix_quick_extract::headers::UserAgent;
use actix_session::Session as ActixSession;
use actix_web::{HttpResponse, dev::PeerAddr, error::ErrorInternalServerError, post, web};
use chrono::{Duration, Utc};

use crate::services::{
    auth::{generate_session_hash, get_number_users, register_session, register_user},
    password,
};
use crate::structs::{
    auth::{PasswordHashAlgorithm, RegisterRequest, RegisterResponse, Session, User, UserRole},
    state::AppState,
};

#[post("/register")]
async fn post_register(
    state: web::Data<AppState>,
    payload: web::Json<RegisterRequest>,
    cache: ActixSession,
    user_agent: UserAgent,
    peer_addr: PeerAddr,
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    // First user to register is owner
    let is_owner: bool = get_number_users(&state.pgpool)
        .await
        .map_err(ErrorInternalServerError)?
        == 0;

    // TODO: add payload validity checks

    let password_hash = password::hash_password(&payload.password).map_err(|err| {
        log::error!("failed to hash password: {err:?}");
        ErrorInternalServerError("failed to hash password")
    })?;
    let session_hash = generate_session_hash().map_err(|err| {
        log::error!("failed to generate session hash: {err:?}");
        ErrorInternalServerError("Failed to generate session hash")
    })?;
    let created_at: chrono::DateTime<Utc> = Utc::now();

    // User creation
    let user_id: i32 = register_user(
        &state.pgpool,
        &User {
            id: -1,
            username: payload.username.clone(),
            display_name: payload.display_name.clone(),
            email_address: payload.email_address.clone(),
            password_hash,
            password_hash_algorithm: PasswordHashAlgorithm::Argon2,
            user_role: if is_owner {
                UserRole::Owner
            } else {
                UserRole::User
            },
        },
    )
    .await
    .map_err(|err| {
        log::error!("failed to register user: {err:?}");
        ErrorInternalServerError("failed to create user")
    })?;

    // Session registration
    register_session(
        &state.pgpool,
        &cache,
        &Session {
            user_id,
            session_hash,
            ip_address: peer_addr.0.ip(),
            user_agent: Some(user_agent.0),
            created_at,
            expires_at: created_at + Duration::hours(24),
            revoked_at: None,
        },
    )
    .await
    .map_err(|err| {
        log::error!("failed to register session: {err:?}");
        ErrorInternalServerError("failed to create session")
    })?;

    Ok(HttpResponse::Created().json(RegisterResponse {
        success: true,
        failure_reason: String::new(),
    }))
}
