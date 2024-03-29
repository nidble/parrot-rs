use serde::Serialize;
use warp::reply::WithStatus;

use crate::{
    schema::{Funtranslations, PokeApi},
    services::Fetcher,
};

#[derive(Serialize)]
struct ApiResponse {
    name: String,
    description: String,
}

pub async fn handle_pokemon(
    (pokemon_fetcher, trans_fetcher): (Fetcher, Fetcher),
    name: String,
) -> std::result::Result<WithStatus<warp::reply::Json>, warp::Rejection> {
    let poke_api: PokeApi = pokemon_fetcher
        .fetch("api/v2/ability/", &name)
        .await
        .map_err(warp::reject::custom)?;

    let text = poke_api
        .get_description()
        .map_err(|_e| warp::reject::not_found())?;

    let funtrans_api: Funtranslations = trans_fetcher
        .fetch("translate/shakespeare.json?text=", &text)
        .await
        .map_err(warp::reject::custom)?;

    let description = funtrans_api.contents.translated;
    let json = warp::reply::json(&ApiResponse { description, name });

    Ok(warp::reply::with_status(json, reqwest::StatusCode::OK))
}
