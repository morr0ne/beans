use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

pub use reqwest::Method;

pub type HttpClient = reqwest::Client;

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
