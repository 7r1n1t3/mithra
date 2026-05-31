use actix_files::{Files, NamedFile};
use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;

mod dto;
mod routes;
mod services;
mod state;

async fn spa_index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./build/200.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DB_URL").expect("DB_URL environment variable must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    let state = state::AppState { pool };

    HttpServer::new(move || {
        App::new()
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
