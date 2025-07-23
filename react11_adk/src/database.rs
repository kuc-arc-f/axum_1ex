use sqlx::{PgPool, Row};
use std::env;

pub type DbPool = PgPool;

pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    //let database_url = env::var("DATABASE_URL")
    //    .unwrap_or_else(|_| "postgresql://user:password@localhost/tododb".to_string());
    let database_url = "postgresql://postgresql:admin@localhost/postgresql".to_string();
    
    PgPool::connect(&database_url).await
}

pub async fn init_db(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            "id" SERIAL NOT NULL,
            title TEXT,
            content TEXT,
            created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
            CONSTRAINT "Todo_pkey" PRIMARY KEY ("id")
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}