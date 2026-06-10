use actix_session::Session as ActixSession;
use sqlx::PgPool;

use crate::auth::{
    error::DatabaseError,
    models::{ID, LoginAttempt, Session, User},
};
use crate::services::password;

pub async fn verify_credentials(
    email: &str,
    password: &str,
    pool: &PgPool,
) -> Result<(ID, bool), sqlx::Error> {
    let user: Option<(ID, String)> = sqlx::query_as(
        r#"SELECT id, password_hash
        FROM users
        WHERE email_address=$1"#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    let Some((user_id, password_hash)) = user else {
        // user not found
        return Ok((-1, false));
    };

    return Ok((
        user_id,
        password::verify_password(password, &password_hash).unwrap_or(false),
    ));
}

pub fn generate_session_hash() -> Result<Vec<u8>, getrandom::Error> {
    let mut session_hash = vec![0_u8; 32];
    getrandom::fill(&mut session_hash)?;
    Ok(session_hash)
}

pub async fn register_user(pool: &PgPool, user: &User) -> Result<ID, DatabaseError> {
    let user_id: ID = sqlx::query_scalar(
        r#"
        INSERT INTO users
        (username, display_name, email_address, password_hash, password_hash_algorithm, user_role)
        OUTPUT user_id
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(&user.username)
    .bind(&user.display_name)
    .bind(&user.email_address)
    .bind(&user.password_hash)
    .bind(&user.password_hash_algorithm)
    .bind(&user.user_role)
    .fetch_one(pool)
    .await?;

    log::info!(
        "Created user {0}[{1}]",
        user_id,
        user.email_address.to_string()
    );

    return Ok(user_id);
}

pub async fn register_session(
    pool: &PgPool,
    cache: &ActixSession,
    session: &Session,
) -> Result<(), DatabaseError> {
    store_session(pool, session).await?;
    cache_session(cache, session)?;
    log::info!("Created session for user {0}", session.user_id);

    Ok(())
}

async fn store_session(pool: &PgPool, session: &Session) -> Result<(), sqlx::Error> {
    let _ = sqlx::query(
        r#"INSERT INTO sessions
        (user_id, session_hash, ip_address, user_agent, created_at, expires_at, revoked_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
    "#,
    )
    .bind(&session.user_id)
    .bind(&session.session_hash)
    .bind(&session.ip_address)
    .bind(&session.user_agent)
    .bind(&session.created_at)
    .bind(&session.expires_at)
    .bind(&session.revoked_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub fn cache_session(
    cache: &ActixSession,
    session: &Session,
) -> Result<(), actix_session::SessionInsertError> {
    cache.insert("session", session)
}

pub async fn store_login_attempt(pool: &PgPool, login_attempt: LoginAttempt) {
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
pub async fn get_number_users(pool: &PgPool) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar(r#"SELECT count(*) FROM users"#)
        .fetch_one(pool)
        .await
}
