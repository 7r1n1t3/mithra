use crate::dto::auth::{SignInRequest, SignInResponse};
use crate::services::password;
use crate::state::AppState;

use actix_web::{HttpResponse, post, web};
use sqlx::PgPool;

#[post("/signin")]
async fn post_signin(
    state: web::Data<AppState>,
    payload: web::Json<SignInRequest>,
) -> actix_web::Result<HttpResponse> {
    match verify_credentials(
        payload.email.as_str(),
        payload.password.as_str(),
        &state.pool,
    )
    .await
    {
        Ok(Some(_user_id)) => Ok(HttpResponse::Ok().json(SignInResponse {
            success: true,
            failure_reason: String::new(),
        })),
        Ok(None) => Ok(HttpResponse::Unauthorized().json(SignInResponse {
            success: false,
            failure_reason: "Invalid email address or password".to_owned(),
        })),
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Sign-in failed")),
    }
}

async fn verify_credentials(
    email: &str,
    password: &str,
    pool: &PgPool,
) -> Result<Option<String>, sqlx::Error> {
    let user: Option<(String, String)> = sqlx::query_as(
        r#"SELECT id, password_hash
        FROM users
        WHERE email_address=$1"#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    let Some((user_id, password_hash)) = user else {
        return Ok(None);
    };

    let is_valid: bool = password::verify_password(password, &password_hash).unwrap_or(false);

    Ok(is_valid.then_some(user_id))
}
