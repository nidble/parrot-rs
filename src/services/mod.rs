use reqwest::{Client, Error};
use std::time::Duration;

pub fn get_client() -> Result<Client, Error> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(10)) // this is for safety and best practice,
        .build()
}
