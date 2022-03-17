use std::net::TcpListener;

use mafi::start;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    start(listener).await?;

    Ok(())
}
