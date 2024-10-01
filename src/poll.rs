use anyhow;
use sqlx::PgPool;
use tracing::debug;

use crate::TOTAL_SUPPLY;

pub async fn poll_total_supply(db_url: &str) -> anyhow::Result<()> {
    let period = crate::POLLING_PERIOD;
    debug!("updating total supply every {} seconds", period);
    let pool = PgPool::connect(db_url).await?;

    let mut height = 0i64;
    loop {
        let current_height: i64 = sqlx::query_scalar("SELECT MAX(height) FROM block_details")
            .fetch_one(&pool)
            .await?;
        tracing::info!("updating total supply {} -> {}", height, current_height);
        let supply: i64 = sqlx::query_scalar(
            r#"
SELECT COALESCE((staked_um + unstaked_um + auction + dex)::BIGINT, 0) as total
FROM (
  SELECT SUM(um) as staked_um
  FROM (
    SELECT * 
    FROM supply_validators
  ) validators
  LEFT JOIN LATERAL (
    SELECT um  
    FROM supply_total_staked
    WHERE validator_id = id 
    AND height >= $1
    AND height < $2
    ORDER BY height DESC 
    LIMIT 1
  ) ON TRUE
) staked
LEFT JOIN LATERAL (
  SELECT um as unstaked_um, auction, dex 
  FROM supply_total_unstaked
  WHERE height >= $1
  AND height < $2
  LIMIT 1
) on TRUE
        "#,
        )
        .bind(height)
        .bind(current_height + 1)
        .fetch_one(&pool)
        .await?;
        {
            let mut total_supply = TOTAL_SUPPLY.lock().await; 
            *total_supply += u128::try_from(supply)?;
        }
        height = current_height;
        tokio::time::sleep(std::time::Duration::from_secs(period)).await;
    }
}
