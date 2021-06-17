use anyhow::Result;
use indexmap::IndexMap;
use reqwest::IntoUrl;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

pub type HttpClient = reqwest::blocking::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetResponse {
    pub args: IndexMap<String, String>,
    pub headers: IndexMap<String, String>,
    pub origin: String,
    pub url: Url,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UuidResponse {
    pub uuid: Uuid,
}

pub struct Client {
    http_client: HttpClient,
}

impl Client {
    pub fn new() -> Self {
        let http_client = HttpClient::new();
        Client { http_client }
    }

    fn get_json<T: DeserializeOwned>(&self, url: impl IntoUrl) -> Result<T> {
        Ok(self.http_client.get(url).send()?.json()?)
    }

    pub fn get(&self) -> Result<GetResponse> {
        self.get_json("https://httpbin.org/get")
    }

    pub fn uuid(&self) -> Result<UuidResponse> {
        self.get_json("https://httpbin.org/uuid")
    }
}
