

use tokio;
mod stream_server;

use std::env;
use env_logger::Builder;

#[tokio::main]
async fn main() {
    Builder::new()
        .parse_env(&env::var("MY_APP_LOG").unwrap_or_default())
        .filter(None, log::LevelFilter::Debug)
        .init();
    stream_server::run("0.0.0.0:8080").await;
}
