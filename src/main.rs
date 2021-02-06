use std::env;

use log::{log, Level};
use warp::{http::Method, http::StatusCode, Filter};

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "parrot-rs=info")
    }
    pretty_env_logger::init();

    let cors = warp::cors().allow_methods(&[Method::GET]);

    let health = warp::path("healthz").map(|| StatusCode::OK);

    let routes = health.with(cors);

    log!(Level::Info, "live & running...");

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await
}
