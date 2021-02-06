use reqwest::{Client, Error, Response};
use std::time::Duration;

pub fn get_client() -> Result<Client, Error> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(10)) // this is for safety and best practice,
        .build()
}

#[derive(Clone)]
pub struct Fetcher {
    client: Client,
    base_url: String,
}

impl Fetcher {
    pub fn new<'a>(client: &Client, base_url: &'a str) -> Self {
        Fetcher {
            client: client.clone(),
            base_url: base_url.into(),
        }
    }

    pub async fn fetch(&self, path: String) -> Result<Response, Error> {
        self.client
            .get(&format!("{}/{}", self.base_url, path))
            .send()
            .await
    }
}
