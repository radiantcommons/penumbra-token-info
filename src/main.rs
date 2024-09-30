use anyhow::anyhow;
use chrono::Utc;
use std::sync::LazyLock;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing_subscriber;

mod accounting;
mod poll;
mod server;

pub type Cache<T> = Arc<Mutex<T>>;

pub static TOTAL_SUPPLY: LazyLock<Cache<u128>> = LazyLock::new(|| Arc::new(Mutex::new(0)));
pub static LOCKED_BALANCES: LazyLock<Cache<HashMap<String, u128>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

pub static POLLING_PERIOD: u64 = 10;
pub static FAILED_QUERY_RETRIES: u64 = 3;

pub static GRPC_ENDPOINTS: [&str; 1] = ["http://localhost:50051"];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    init_static_balances().await;

    let server_task: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
        server::listen("0.0.0.0:3000".parse()?).await;
        Ok(())
    });

    let args = std::env::args().into_iter().collect::<Vec<_>>();
    let db_url = args
        .get(1)
        .ok_or(anyhow!("expected <DB_URL> as first argument"))?
        .clone();

    let poll_task: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
        poll::poll_total_supply(db_url.as_str()).await?;
        Ok(())
    });

    poll_task.await??;
    server_task.await??;

    Ok(())
}

//Remove this one we move to static balances
async fn init_static_balances() {
    for fvk in accounting::PENUMBRA_LABS_FVKS.iter() {
        let mut locked_balances = LOCKED_BALANCES.lock().await;
        locked_balances.insert(fvk.to_string(), accounting::PENUMBRA_LABS_BALANCE);
    }

    for fvk in accounting::RADIANT_FVKS.iter() {
        let mut locked_balances = LOCKED_BALANCES.lock().await;
        locked_balances.insert(fvk.to_string(), accounting::RADIANT_BALANCE);
    }

    for fvk in accounting::NUMOGRAPHICA_FVKS.iter() {
        let mut locked_balances = LOCKED_BALANCES.lock().await;
        locked_balances.insert(fvk.to_string(), accounting::NUMOGRAPHICA_BALANCE);
    }

    let mut locked_balances = LOCKED_BALANCES.lock().await;
    locked_balances.insert(
        "INVESTOR_LOCKED_SUPPLY".to_string(),
        accounting::investor_locked_supply(Utc::now()),
    );
}
