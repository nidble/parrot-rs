use reqwest::{Client, Error, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::time::Duration;

#[derive(Debug)]
pub enum FetcherError {
    FetchError(String),
}
impl warp::reject::Reject for FetcherError {}

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

    async fn get(&self, path: String) -> Result<Response, Error> {
        self.client
            .get(&format!("{}/{}", self.base_url, path))
            .send()
            .await
    }

    pub async fn fetch<T>(&self, path: &str, text: &str) -> Result<T, FetcherError>
    where
        T: DeserializeOwned,
    {
        let resp = self
            .get(format!("{}{}", path, text))
            .await
            .map_err(|e| FetcherError::FetchError(e.to_string()))?;

        match resp.status() {
            StatusCode::OK => resp
                .json::<T>()
                .await
                .map_err(|e| FetcherError::FetchError(e.to_string())),
            s => Err(FetcherError::FetchError(format!(
                "KO 👎, [{}] {}!",
                self.base_url, s
            ))),
        }
    }
}
