use crate::dto::auth::{RegisterRequest, RegisterResponse, SignInRequest, SignInResponse};
use crate::services::password;
use crate::state::AppState;

use actix_files::NamedFile;
use actix_web::{HttpResponse, Result, error, get, post, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_register);
    cfg.service(get_signin);
    cfg.service(post_register);
    cfg.service(post_signin);
}

#[get("/register")]
async fn get_register() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/register/register.html")?)
}

#[get("/singin")]
async fn get_signin() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/signin/signin.html")?)
}

#[post("/register")]
async fn post_register(
    state: web::Data<AppState>,
    payload: web::Json<RegisterRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let password_hash =
        password::hash_password(&payload.password).map_err(error::ErrorInternalServerError)?;

    sqlx::query(
        r#"
        INSERT INTO users
        (username, display_name, email, password_hash, password_hash_algorithm)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(&payload.username)
    .bind(&payload.display_name)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind("argon2")
    .fetch_one(&state.pool)
    .await
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(RegisterResponse {
        success: true,
        username: payload.username.clone(),
        failure_reason: String::new(),
    }))
}

#[post("/signin")]
async fn post_signin(
    _state: web::Data<AppState>,
    _payload: web::Json<SignInRequest>,
) -> Result<web::Json<SignInResponse>, actix_web::Error> {
    Ok(web::Json(SignInResponse {
        success: false,
        failure_reason: "sign-in not implemented yet".to_string(),
    }))
}
