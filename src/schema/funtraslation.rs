use serde::Deserialize;

#[derive(Deserialize)]
pub struct Funtranslations {
    pub contents: Contents,
}

#[derive(Deserialize)]
pub struct Contents {
    pub translated: String,
}
