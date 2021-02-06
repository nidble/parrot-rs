pub mod error;
pub mod pokemon;

use error::handle_rejection;
use pokemon::handle_pokemon;
use std::convert::Infallible;
use warp::{http::Method, http::StatusCode, Filter};

use crate::services::Fetcher;

fn pokemon(
    services: (Fetcher, Fetcher),
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GET /pokemon/:string
    warp::path("pokemon")
        .and(warp::any().map(move || (services.clone())))
        .and(warp::path::param())
        .and_then(handle_pokemon)
}

pub fn api(
    services: (Fetcher, Fetcher),
) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    let cors = warp::cors().allow_methods(&[Method::GET]);
    let health = warp::path("healthz").map(|| StatusCode::OK);
    let routes = health.or(pokemon(services)).with(cors);

    routes.recover(handle_rejection)
}
