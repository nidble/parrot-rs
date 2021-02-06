use serde::Serialize;
use warp::reply::WithStatus;

#[derive(Serialize)]
struct ApiResponse {
    name: String,
    description: String,
}

pub async fn handle_pokemon(
    name: String,
) -> std::result::Result<WithStatus<warp::reply::Json>, warp::Rejection> {
    let description = "Hello pokemon".to_string();
    let json = warp::reply::json(&ApiResponse { description, name });

    Ok(warp::reply::with_status(json, reqwest::StatusCode::OK))
}
