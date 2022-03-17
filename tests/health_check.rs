use common::{TesterInner, Tester};
use tokio::test;

mod common;

#[test]
async fn ping_returns_pong() {
    let tester = Tester::new();
    let response = tester.get("/ping").await;
    assert_eq!(response.get_body(), "pong");
}
