use std::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn create_db_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}

pub async fn insert_user(db: &PgPool, user_id: &str, email_hash: &str, password_hash: &str) -> Result<(), sqlx::Error> {
    // sqlx::query!(
    //     r#"
    //     INSERT INTO users (id, email, password)
    //     VALUES ($1, $2, $3)
    //     "#,
    //     user_id,
    //     email_hash,
    //     password_hash
    // )
    // .execute(db)
    // .await?;

    Ok(())
}