use std::env;

use log::{log, Level};
mod filters;
use filters::api;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "parrot-rs=info")
    }
    pretty_env_logger::init();

    log!(Level::Info, "live & running...");

    warp::serve(api()).run(([0, 0, 0, 0], 3030)).await
}
