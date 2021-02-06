mod filters;
mod schema;
mod services;

use std::env;

use filters::api;
use log::{log, Level};
use services::{get_client, Fetcher};

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "parrot-rs=info")
    }
    pretty_env_logger::init();

    let client = get_client().expect("Client initialization failed");
    let services = (
        Fetcher::new(&client, "https://pokeapi.co"),
        Fetcher::new(&client, "https://api.funtranslations.com"),
    );

    log!(Level::Info, "live & running...");
    warp::serve(api(services)).run(([0, 0, 0, 0], 3030)).await
}
