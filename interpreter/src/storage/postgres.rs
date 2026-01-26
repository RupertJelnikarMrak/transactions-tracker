use sqlx::{PgPool, postgres::PgPoolOptions};

pub struct PgRepo {
    pool: PgPool,
}

impl PgRepo {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new().connect(url).await?;
        Ok(Self { pool })
    }

    pub async fn insert_transaction(
        &self,
        sig: &str,
        slot: u64,
        user: &str,
        mint: &str,
        amount: f64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO transactions (signature, slot, user_address, token_mint, amount)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (signature) DO NOTHING
            "#,
            sig,
            slot as i64,
            user,
            mint,
            amount
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
