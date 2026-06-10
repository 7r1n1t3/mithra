use actix_files::{Files, NamedFile};
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::{App, HttpServer, cookie::Key, middleware::Logger, web};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use log::info;
use sqlx::postgres::PgPoolOptions;

mod routes;
mod services;
mod structs;

async fn spa_index() -> actix_web::Result<NamedFile> {
    // svelte fallback page
    Ok(NamedFile::open("./build/200.html")?)
}

fn load_session_key() -> Key {
    let encoded =
        std::env::var("SESSION_SECRET").expect("SESSION_SECRET environment variable must be set");

    let bytes = STANDARD
        .decode(encoded.trim())
        .expect("SESSION_SECRET must be valid base64");

    Key::try_from(bytes.as_slice()).expect("SESSION_SECRET must decode to at least 64 random bytes")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Postgres
    let postgres_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
    let pgpool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Redis
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL environment variable is missing");
    let redis_store = RedisSessionStore::new(redis_url).await.unwrap();
    // When using `Key::generate()` it is important to initialize outside of the
    // `HttpServer::new` closure. When deployed the secret key should be read from a
    // configuration file or environment variables.
    let secret_key: Key = load_session_key();
    let state = structs::state::AppState { pgpool };

    info!("Starting HTTP server");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%t %a %{User-Agent} %r").exclude_regex(r"^/_app(?:/|$)"))
            // Add session management using Redis for session state storage
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(state.clone()))
            .configure(routes::configure)
            .service(
                Files::new("/", "./build")
                    .index_file("200.html")
                    .default_handler(web::to(spa_index)),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
