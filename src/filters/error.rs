use reqwest::StatusCode;
use serde::Serialize;
use std::convert::Infallible;
use warp::{Rejection, Reply};

use crate::services::FetcherError;

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "not found";
    } else if let Some(fetch_error) = err.find::<FetcherError>() {
        code = fetch_error.get_status();
        message = fetch_error.get_message();
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "method not allowed";
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "unhandled rejection";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
