use std::net::TcpListener;

use mafi::start;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = std::env::var("MAFI_PORT").unwrap_or("3000".to_string());
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address).expect(&format!("Could not bind to {}", address));

    start(listener).await?;

    Ok(())
}
