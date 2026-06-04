use crate::services::password;
use sqlx::PgPool;

pub async fn verify_credentials(
    email: &str,
    password: &str,
    pool: &PgPool,
) -> Result<(i32, bool), sqlx::Error> {
    let user: Option<(i32, String)> = sqlx::query_as(
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
