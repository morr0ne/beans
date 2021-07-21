use anyhow::Result;
use reqwest::{IntoUrl, Method};
use serde::de::DeserializeOwned;
use types::{GetResponse, UuidResponse};

pub mod types;
pub use types::HttpClient;

pub struct Client {
    http_client: HttpClient,
}

impl Client {
    pub fn new() -> Self {
        let http_client = HttpClient::new();
        Client { http_client }
    }

    pub async fn request_json<T: DeserializeOwned>(
        &self,
        method: Method,
        url: impl IntoUrl,
    ) -> Result<T> {
        Ok(self
            .http_client
            .request(method, url)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn get(&self) -> Result<GetResponse> {
        self.request_json(Method::GET, "https://httpbin.org/get")
            .await
    }

    pub async fn uuid(&self) -> Result<UuidResponse> {
        self.request_json(Method::GET, "https://httpbin.org/uuid")
            .await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
