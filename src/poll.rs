use tokio_retry::{
    strategy::{jitter, ExponentialBackoff},
    Retry,
};
use tracing::{debug, error, warn};

pub async fn poll_total_supply() -> Result<(), String> {
    let period = crate::POLLING_PERIOD;
    debug!("updating total supply every {} seconds", period);

    // jittered retry with exponential backoff
    let retry_strategy = ExponentialBackoff::from_millis(500)
        .map(jitter)
        .take(crate::FAILED_QUERY_RETRIES as usize);
    loop {
        debug!("updating total supply");
        Retry::spawn(retry_strategy.clone(), || async {
            for endpoint in crate::GRPC_ENDPOINTS.iter() {
                warn!("querying endpoint {}", endpoint);
                // query the endpoint
                // let supply = query_total_supply(endpoint).await?;
                // let mut total_supply = crate::TOTAL_SUPPLY.lock().await;
                // *total_supply = supply;
            }
            return Ok(());
        })
        .await
        .unwrap_or_else(|e: String| error!("{:?}", e));
        tokio::time::sleep(std::time::Duration::from_secs(period)).await;
    }
}
