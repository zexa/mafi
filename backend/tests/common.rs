use std::net::TcpListener;

use async_trait::async_trait;
use hyper::{Client, client::HttpConnector, Response, Body, Request};
use mafi::get_router;
use tokio::task::JoinHandle;

#[async_trait]
pub trait TesterInner {
    async fn build_tester_response(&self, response: Response<Body>) -> TesterResponse;
    async fn get(&self, path: &str) -> TesterResponse;
    async fn post(&self, path: &str, body: Option<String>) -> TesterResponse;
}

pub struct Tester {
    client: Client<HttpConnector>,
    handle: JoinHandle<()>,
    address: String,
}

impl Tester {
    fn spawn_app() -> (JoinHandle<()>, String) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind listener");
        let port = listener.local_addr().expect("Could not get local addr from listener").port();
        let handle = tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .expect("Could not build axum server")
                .serve(get_router().into_make_service())
                .await
                .expect("Server failed")
        });
        let address = format!("http://127.0.0.1:{}", port);
        
        (handle, address)
    }

    pub fn new() -> Self {
        let (handle, address) = Self::spawn_app();

        Self {
            client: hyper::Client::new(), 
            handle,
            address,
        }
    }
}

impl Drop for Tester {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

#[async_trait]
impl TesterInner for Tester {
    async fn build_tester_response(&self, response: Response<Body>) -> TesterResponse {
        let status_code = response.status().as_u16();
        let body = hyper::body::to_bytes(response.into_body()).await.expect("Could not get response body bytes");
        let body = std::str::from_utf8(&body[..]).expect("Could not convert body response to &str").to_string();
        
        TesterResponse::new(status_code, body)
    }


    async fn get(&self, path: &str) -> TesterResponse {
        let response = self.client
            .request(
                Request::builder()
                    .header("Content-Type", "application/json")
                    .uri(format!("{}{}", self.address, path))
                    .body(Body::empty())
                    .expect("Could not build request")
            )
            .await
            .expect("Could not complete request");

        self.build_tester_response(response).await
    }

    async fn post(&self, path: &str, body: Option<String>) -> TesterResponse {
        let body = match body {
            None => Body::empty(),
            Some(body) => body.into(),
        };

        let response = self.client
            .request(
                Request::builder()
                    .header("Content-Type", "application/json")
                    .method("POST")
                    .uri(format!("{}{}", self.address, path))
                    .body(body)
                    .expect("Could not build request")
            )
            .await
            .expect("Could not complete request");

        self.build_tester_response(response).await
    }
}

#[derive(Debug)]
pub struct TesterResponse {
    status_code: u16,
    body: String,
}

impl TesterResponse {
    pub fn new(status_code: u16, body: String) -> Self {
        Self {
            status_code,
            body,
        }
    }

    pub fn get_status_code(&self) -> &u16 {
        &self.status_code
    }

    pub fn get_body(&self) -> &str {
        &self.body
    }
}
