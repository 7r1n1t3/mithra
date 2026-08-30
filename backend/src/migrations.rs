use diesel_async::{AsyncPgConnection, async_connection_wrapper::AsyncConnectionWrapper};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use log::info;

/// The `.sql` files under `backend/migrations/` are baked into the binary at compile time, so the
/// runtime image needs no migration files of its own.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Applies every migration the database has not seen yet, in directory-name order.
///
/// Diesel records what it has applied in a `__diesel_schema_migrations` table it creates itself,
/// so this is a no-op on an already up-to-date database and safe to call on every startup.
pub async fn run(database_url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use diesel_async::AsyncConnection;

    let conn = AsyncPgConnection::establish(database_url).await?;

    // The migration harness is a blocking API even on an async connection: the wrapper drives the
    // connection's futures to completion internally, so it has to run off the async workers.
    let mut conn = AsyncConnectionWrapper::<AsyncPgConnection>::from(conn);
    let applied = actix_web::rt::task::spawn_blocking(move || {
        conn.run_pending_migrations(MIGRATIONS)
            .map(|versions| versions.iter().map(ToString::to_string).collect::<Vec<_>>())
    })
    .await??;

    if applied.is_empty() {
        info!("Database schema is up to date");
    } else {
        for version in &applied {
            info!("Applied migration {version}");
        }
    }

    Ok(())
}
