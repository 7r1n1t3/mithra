use crate::dto::auth::{RegisterRequest, RegisterResponse};
use crate::services::password;
use crate::state::AppState;

use actix_web::{HttpResponse, error, post, web};

#[post("/register")]
async fn post_register(
    state: web::Data<AppState>,
    payload: web::Json<RegisterRequest>,
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    let password_hash =
        password::hash_password(&payload.password).map_err(error::ErrorInternalServerError)?;

    let result = sqlx::query(
        r#"
        INSERT INTO users
        (username, display_name, email_address, password_hash, password_hash_algorithm)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(&payload.username)
    .bind(&payload.display_name)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind("argon2")
    .execute(&state.pool)
    .await
    .map_err(error::ErrorInternalServerError)?;

    if result.rows_affected() == 0 {
        return Ok(HttpResponse::Created().json(RegisterResponse {
            success: false,
            username: payload.username.clone(),
            failure_reason: format!("{0} is already registered.", payload.username).to_string(),
        }));
    }
    Ok(HttpResponse::Created().json(RegisterResponse {
        success: true,
        username: payload.username.clone(),
        failure_reason: String::new(),
    }))
}
