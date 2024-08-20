use sqlx::{Pool, Postgres};
use std::error::Error;

pub async fn delete_count(id: i64, pool: Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    let transaction: sqlx::Transaction<'_, Postgres> = pool.begin().await?;

    let delete_all: &str = "
    WITH deleted_counters AS (
        DELETE FROM counters_metrika
        WHERE counter_id = $1
        RETURNING counter_id
    )
    DELETE FROM goals
    WHERE fk_counters_metrika_counter_id IN (SELECT counter_id FROM deleted_counters);";

    sqlx::query(&delete_all).bind(id).execute(&pool).await?;

    transaction.commit().await?;

    Ok(())
}
