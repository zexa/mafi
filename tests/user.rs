use common::{TesterInner, Tester, TesterResponse};
use tokio::test;

mod common;

async fn register_user_valid() -> TesterResponse {
    let tester = Tester::new();
    let payload = "{\"name\": \"zexa\"}";

    tester.post("/user", Some(payload.to_string())).await
}

#[test]
async fn register_user_returns_ok_status_code() {
    let response = register_user_valid().await;

    assert_eq!(response.get_status_code(), &200_u16, "{:?}", response);
}

#[test]
async fn register_user_returns_valid_json() {
    let response = register_user_valid().await;
    // let json = serde_json::from_str(response.get_body());



    // assert_eq!(response.get_status_code(), &200_u16, "{:?}", response);
}