use actix_files::{Files, NamedFile};
use actix_web::{App, HttpResponse, HttpServer, Result, error, get, post, web};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};

mod auth;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    username: String,
    display_name: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    success: bool,
    username: String,
    failure_reason: String,
}

#[derive(Debug, Deserialize)]
struct SignInRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct SignInResponse {
    success: bool,
    failure_reason: String,
}

/// root
#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

/// register
#[get("/register")]
async fn register() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/register/register.html")?)
}

/// register
#[get("/singin")]
async fn signin() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/signin/signin.html")?)
}

/// API
/// Register
#[post("/api/register")]
async fn post_register(
    state: web::Data<AppState>,
    payload: web::Json<RegisterRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let password_hash = auth::password::hash_password(&payload.password)
        .map_err(error::ErrorInternalServerError)?;

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

/// Sign-in
#[post("/api/signin")]
async fn post_signin(
    state: web::Data<AppState>,
    payload: web::Json<SignInRequest>,
) -> Result<web::Json<SignInResponse>, actix_web::Error> {
    Ok(web::Json(SignInResponse {
        success: false,
        failure_reason: "sign-in not implemented yet".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DB_URL").expect("DB_URL environment variable must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    let state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(index)
            .service(register)
            .service(signin)
            .service(post_register)
            .service(post_signin)
            .service(Files::new("/static", "./static"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
