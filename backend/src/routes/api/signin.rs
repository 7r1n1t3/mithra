use actix_quick_extract::headers::UserAgent;
use actix_session::Session as ActixSession;
use actix_web::dev::PeerAddr;
use actix_web::{HttpResponse, error, post, web};
use chrono::{Duration, Utc};

use crate::services::auth::{
    generate_session_hash, register_session, store_login_attempt, verify_credentials,
};
use crate::structs::{
    auth::{LoginAttempt, Session, SignInRequest, SignInResponse},
    state::AppState,
};

#[post("/signin")]
async fn post_signin(
    state: web::Data<AppState>,
    payload: web::Json<SignInRequest>,
    cache: ActixSession,
    user_agent: UserAgent,
    peer_addr: PeerAddr,
) -> actix_web::Result<HttpResponse> {
    // TODO: check if session is registered, then return success
    let ip_address = peer_addr.0.ip();
    let attempted_at: chrono::DateTime<Utc> = Utc::now();
    let expires_at: chrono::DateTime<Utc> = attempted_at + Duration::hours(24);

    match verify_credentials(
        payload.email_address.as_str(),
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
            log::info!(
                "failed login for user {}[{}]",
                payload.email_address,
                user_id
            );
            let failure_reason = String::from("Invalid email address or password");

            store_login_attempt(
                &state.pgpool,
                LoginAttempt {
                    user_id,
                    // This should never fail: IP check was done before
                    ip_address,
                    user_agent: Some(user_agent.0),
                    success: false,
                    attempted_at,
                    failure_reason: failure_reason.clone(),
                },
            )
            .await;

            return Ok(HttpResponse::Unauthorized().json(SignInResponse {
                success: false,
                failure_reason: failure_reason,
            }));
        }
        // Successful login
        Ok((user_id, true)) => {
            log::info!(
                "successful login for user {}[{}]",
                payload.email_address,
                user_id
            );

            store_login_attempt(
                &state.pgpool,
                LoginAttempt {
                    user_id,
                    ip_address,
                    user_agent: Some(user_agent.0.clone()),
                    success: true,
                    attempted_at,
                    failure_reason: String::new(),
                },
            )
            .await;

            register_session(
                &state.pgpool,
                &cache,
                &Session {
                    user_id,
                    session_hash: generate_session_hash().map_err(|_| {
                        error::ErrorInternalServerError("Failed to generate session hash")
                    })?,
                    ip_address,
                    user_agent: Some(user_agent.0),
                    created_at: attempted_at,
                    expires_at,
                    revoked_at: None,
                },
            )
            .await
            .map_err(|err| {
                log::error!("failed to register session: {err:?}");
                error::ErrorInternalServerError("failed to create session")
            })?;

            return Ok(HttpResponse::Ok().json(SignInResponse {
                success: true,
                failure_reason: String::new(),
            }));
        }
    }
}
