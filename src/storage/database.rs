use std::time::Duration;

use sqlx::{Connection, PgPool, postgres::PgPoolOptions};

pub struct Database {
    db: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        // let db = PgPool::connect(database_url).await?;
        let db = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(3))
            .connect(database_url)
            .await?;
        Ok(Self { db })
    }

    pub fn connection(&self) -> &PgPool {
        &self.db
    }

    pub async fn ping(&self) -> Result<(), sqlx::Error> {
        let mut conn = self.db.acquire().await?;
        conn.ping().await
    }
}
