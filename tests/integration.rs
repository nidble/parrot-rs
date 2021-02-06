use parrot_rs::filters;
use parrot_rs::services;

#[cfg(test)]
mod tests {
    // use warp::http::StatusCode;
    use warp::test::request;
    // use warp::Filter;

    use super::filters::api;
    use super::services::{get_client, Fetcher};

    use httpmock::MockServer;
    use serde_json::json;

    #[tokio::test]
    async fn all_service_return_ok_test() {
        // Arrange
        let server = MockServer::start();
        let mock1 = server.mock(|when, then| {
            when.method("GET")
                .path("/api/v2/ability/42");
            then.status(200)
                .json_body(json!({
                    "effect_entries": [
                        {
                            "effect": "Mit jedem Treffer besteht eine 10% Chance das Ziel zurückschrecken zu lassen.",
                            "language": { "name": "de" }
                        },
                        {
                            "effect": "Has a 10% chance of making target Pokémon flinch with each hit.",
                            "language": { "name": "en"  }
                        }
                    ],
                }));
        });

        let mock2 = server.mock(|when, then| {
            when.method("GET").path("/translate/shakespeare.json");
            then.status(200).json_body(
                json!({ "success": { "total": 1 }, "contents": { "translated": "foobar 42" } }),
            );
        });

        let client = get_client().expect("client not available");
        let services = (
            Fetcher::new(&client, &server.base_url()),
            Fetcher::new(&client, &server.base_url()),
        );
        let filter = api(services);

        let res = request().path("/pokemon/42").reply(&filter).await;

        // Assert
        mock1.assert();
        mock2.assert();
        assert_eq!(res.status(), 200, "GET works");
    }
}
