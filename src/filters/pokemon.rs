use reqwest::StatusCode;
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
    let pokemon_resp = fetch_pokeapi
        .fetch(format!("api/v2/ability/{}", name))
        .await
        .map_err(|_e| warp::reject::reject())?;

    let poke_api = match pokemon_resp.status() {
        StatusCode::OK => pokemon_resp
            .json::<PokeApi>()
            .await
            .map_err(|_e| warp::reject::reject()),
        _s => todo!(),
    }?;

    let description = poke_api
        .effect_entries
        .iter()
        .find(|e| e.language.name == "en")
        .map(|e| e.effect.replace("\n", " "))
        .unwrap();

    let fetch_funtrans = Fetcher::new(&client, "https://api.funtranslations.com");
    let funtrans_resp = fetch_funtrans
        .fetch(format!("translate/shakespeare.json?text={}", description))
        .await
        .map_err(|_e| warp::reject::reject())?;

    let funtrans_api = match funtrans_resp.status() {
        StatusCode::OK => funtrans_resp
            .json::<Funtranslations>()
            .await
            .map_err(|_e| warp::reject::reject()),
        _s => todo!(),
    }?;

    let description = funtrans_api.contents.translated;
    let json = warp::reply::json(&ApiResponse { description, name });

    Ok(warp::reply::with_status(json, reqwest::StatusCode::OK))
}
