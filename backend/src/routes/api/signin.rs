use crate::dto::auth::{LoginAttempt, Session, SignInRequest, SignInResponse};
use crate::services::auth::verify_credentials;
use crate::state::AppState;
use actix_session::Session as ActixSession;

use actix_web::{HttpResponse, post, web};
use chrono::{Duration, Utc};
use sqlx::PgPool;

#[post("/signin")]
async fn post_signin(
    state: web::Data<AppState>,
    payload: web::Json<SignInRequest>,
    cache: ActixSession,
) -> actix_web::Result<HttpResponse> {
    // TODO: check if session is registered, then return success

    match verify_credentials(
        payload.email.as_str(),
        payload.password.as_str(),
        &state.pgpool,
    )
    .await
    {
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Sign-in failed")),
        Ok((-1, _)) => {
            let failure_reason = String::from("User not found.");
            return Ok(HttpResponse::Unauthorized().json(SignInResponse {
                success: false,
                failure_reason: failure_reason.to_owned(),
            }));
        }
        Ok((user_id, false)) => {
            let failure_reason = String::from("Invalid email address or password");
            let login_attempt: LoginAttempt = LoginAttempt {
                user_id: user_id,
                ip_address: payload.ip_address,
                user_agent: payload.user_agent.clone(),
                success: false,
                attempted_at: Utc::now(),
                failure_reason: failure_reason.clone(),
            };

            store_login_attempt(&state.pgpool, login_attempt).await;

            return Ok(HttpResponse::Unauthorized().json(SignInResponse {
                success: false,
                failure_reason: failure_reason,
            }));
        }
        Ok((user_id, true)) => {
            let login_attempt: LoginAttempt = LoginAttempt {
                user_id: user_id,
                ip_address: payload.ip_address,
                user_agent: payload.user_agent.clone(),
                success: true,
                attempted_at: Utc::now(),
                failure_reason: String::new(),
            };

            store_login_attempt(&state.pgpool, login_attempt).await;

            let session = Session {
                user_id,
                session_hash: generate_session_hash().map_err(|_| {
                    actix_web::error::ErrorInternalServerError("Failed to generate session hash")
                })?,
                ip_address: payload.ip_address,
                user_agent: Some(payload.user_agent.clone()),
                created_at: Utc::now(),
                expires_at: Utc::now() + Duration::hours(24),
                revoked_at: None,
            };

            cache_session(cache, &session)?;
            store_session(&state.pgpool, session).await;

            Ok(HttpResponse::Ok().json(SignInResponse {
                success: true,
                failure_reason: String::new(),
            }))
        }
    }
}

async fn store_login_attempt(pool: &PgPool, login_attempt: LoginAttempt) {
    let _ = sqlx::query(
        r#"INSERT INTO login_attempts
        (user_id, ip_address, user_agent, success, attempted_at, failure_reason)
        VALUES ($1, $2, $3, $4, $5, $6)
    "#,
    )
    .bind(login_attempt.user_id)
    .bind(login_attempt.ip_address)
    .bind(login_attempt.user_agent)
    .bind(login_attempt.success)
    .bind(login_attempt.attempted_at)
    .bind(login_attempt.failure_reason)
    .execute(pool)
    .await;
}

async fn store_session(pool: &PgPool, session: Session) {
    let _ = sqlx::query(
        r#"INSERT INTO sessions
        (user_id, session_hash, ip_address, user_agent, created_at, expires_at, revoked_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
    "#,
    )
    .bind(session.user_id)
    .bind(session.session_hash)
    .bind(session.ip_address)
    .bind(session.user_agent)
    .bind(session.created_at)
    .bind(session.expires_at)
    .bind(session.revoked_at)
    .execute(pool)
    .await;
}

fn cache_session(
    cache: ActixSession,
    session: &Session,
) -> Result<(), actix_session::SessionInsertError> {
    cache.insert("session", session)
}

fn generate_session_hash() -> Result<Vec<u8>, getrandom::Error> {
    let mut session_hash = vec![0_u8; 32];
    getrandom::fill(&mut session_hash)?;
    Ok(session_hash)
}
