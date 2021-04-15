mod filters;
mod schema;
mod services;

use std::env;

use filters::api;
use log::{log, Level};
use services::{get_client, Fetcher};

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;


// TODO: considering an env file
const POKEAPI_BASE_URL: &'static str = "https://pokeapi.co";
const TRANSLATION_BASE_URL: &'static str = "https://api.funtranslations.com";

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "parrot_rs=info")
    }
    pretty_env_logger::init();

    let client = get_client().expect("Client initialization failed");
    let services = (
        Fetcher::new(&client, POKEAPI_BASE_URL),
        Fetcher::new(&client, TRANSLATION_BASE_URL),
    );

    log!(Level::Info, "live & running...");
    warp::serve(api(services)).run(([0, 0, 0, 0], 3030)).await
}
