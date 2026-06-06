use actix_web::{HttpResponse, error, post, web};
use log::info;

use crate::dto::auth::{PasswordHashAlgorithm, RegisterRequest, RegisterResponse, UserRole};
use crate::dto::state::AppState;
use crate::services::password;
#[post("/register")]
async fn post_register(
    state: web::Data<AppState>,
    payload: web::Json<RegisterRequest>,
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    // First user to register is owner
    let is_owner: bool = sqlx::query_scalar(r#"SELECT NOT EXISTS (SELECT 1 FROM users)"#)
        .fetch_one(&state.pgpool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    // TODO: add payload validity checks
    let password_hash =
        password::hash_password(&payload.password).map_err(error::ErrorInternalServerError)?;

    let register_result = sqlx::query(
        r#"
        INSERT INTO users
        (username, display_name, email_address, password_hash, password_hash_algorithm, user_role)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(&payload.username)
    .bind(&payload.display_name)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(PasswordHashAlgorithm::Argon2)
    .bind(
        is_owner
            .then_some(UserRole::Owner)
            .unwrap_or(UserRole::User),
    )
    .execute(&state.pgpool)
    .await
    .map_err(error::ErrorInternalServerError)?;

    if register_result.rows_affected() == 0 {
        info!("user {} successfully registered", payload.email);
        return Ok(HttpResponse::Created().json(RegisterResponse {
            success: false,
            username: payload.username.clone(),
            failure_reason: format!("{} is already registered.", payload.username).to_string(),
        }));
    }
    Ok(HttpResponse::Created().json(RegisterResponse {
        success: true,
        username: payload.username.clone(),
        failure_reason: String::new(),
    }))
}
