use serde::Serialize;
use warp::reply::WithStatus;

use crate::{
    schema::{Funtranslations, PokeApi},
    services::{get_client, Fetcher},
};

#[derive(Serialize)]
struct ApiResponse {
    name: String,
    description: String,
}

pub async fn handle_pokemon(
    name: String,
) -> std::result::Result<WithStatus<warp::reply::Json>, warp::Rejection> {
    let client = get_client().map_err(|_e| warp::reject::reject())?;

    let fetch_pokeapi = Fetcher::new(&client, "https://pokeapi.co");
    let poke_api: PokeApi = fetch_pokeapi
        .fetch("api/v2/ability/", &name)
        .await
        .map_err(|e| warp::reject::custom(e))?;

    let text = poke_api
        .effect_entries
        .iter()
        .find(|e| e.language.name == "en")
        .map(|e| e.effect.replace("\n", " "))
        .unwrap();

    let fetch_funtrans = Fetcher::new(&client, "https://api.funtranslations.com");
    let funtrans_api: Funtranslations = fetch_funtrans
        .fetch("translate/shakespeare.json?text=", &text)
        .await
        .map_err(|e| warp::reject::custom(e))?;

    let description = funtrans_api.contents.translated;
    let json = warp::reply::json(&ApiResponse { description, name });

    Ok(warp::reply::with_status(json, reqwest::StatusCode::OK))
}
