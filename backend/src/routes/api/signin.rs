use std::net::IpAddr;
use std::str::FromStr;

use actix_quick_extract::headers::UserAgent;
use actix_session::Session as ActixSession;
use actix_web::{HttpResponse, dev::ConnectionInfo, post, web};
use chrono::{Duration, Utc};
use log::{error, info};
use sqlx::PgPool;

use crate::dto::{
    auth::{LoginAttempt, Session, SignInRequest, SignInResponse},
    state::AppState,
};
use crate::services::auth::verify_credentials;

#[post("/signin")]
async fn post_signin(
    state: web::Data<AppState>,
    payload: web::Json<SignInRequest>,
    cache: ActixSession,
    user_agent: UserAgent,
    conn_info: ConnectionInfo,
) -> actix_web::Result<HttpResponse> {
    // TODO: check if session is registered, then return success

    // No IP
    if conn_info.peer_addr().is_none() || IpAddr::from_str(conn_info.peer_addr().unwrap()).is_err()
    {
        error!("IP not provided or is not valid");
        let failure_reason = String::from("IP not provided or is not valid. Who are you? Neo?");
        return Ok(HttpResponse::Unauthorized().json(SignInResponse {
            success: false,
            failure_reason: failure_reason,
        }));
    }
    match verify_credentials(
        payload.email.as_str(),
        payload.password.as_str(),
        &state.pgpool,
    )
    .await
    {
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Sign-in failed")),

        // Not found
        Ok((-1, _)) => {
            let failure_reason = String::from("User not found.");
            return Ok(HttpResponse::NotFound().json(SignInResponse {
                success: false,
                failure_reason: failure_reason,
            }));
        }
        // False credentials
        Ok((user_id, false)) => {
            info!("failed login for user {}[{}]", payload.email, user_id);
            let failure_reason = String::from("Invalid email address or password");
            let login_attempt: LoginAttempt = LoginAttempt {
                user_id: user_id,
                // This should never fail: IP check was done before
                ip_address: IpAddr::from_str(
                    conn_info.peer_addr().expect("IP Address not provided."),
                )
                .expect("received IP is not valid"),
                user_agent: user_agent.0,
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
        // Successful login
        Ok((user_id, true)) => {
            info!("successful login for user {}[{}]", payload.email, user_id);
            let login_attempt: LoginAttempt = LoginAttempt {
                user_id: user_id,
                ip_address: payload.ip_address,
                user_agent: user_agent.0.clone(),
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
                user_agent: Some(user_agent.0),
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
