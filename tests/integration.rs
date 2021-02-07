use parrot_rs::filters;
use parrot_rs::services;

#[cfg(test)]
mod tests {
    use warp::test::request;

    use super::filters::api;
    use super::services::{get_client, Fetcher};

    use httpmock::MockServer;
    use serde_json::json;

    #[tokio::test]
    async fn should_return_200() {
        // Arrange
        let server = MockServer::start();
        let mock1 = server.mock(|when, then| {
            when.method("GET").path("/api/v2/ability/42");
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

    #[tokio::test]
    async fn should_return_500() {
        // Arrange
        let server = MockServer::start();
        let mock1 = server.mock(|when, then| {
            when.method("GET").path("/api/v2/ability/42");
            then.status(200)
                .json_body(json!({
                    "effect_entries": [
                        {
                            "effect": "Mit jedem Treffer besteht eine 10% Chance das Ziel zurückschrecken zu lassen.",
                            "language": { "name": "de" }
                        },
                        {
                            "language": { "name": "en"  }
                        }
                    ],
                }));
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
        assert_eq!(res.status(), 500, "GET not works if pokeapi is broken");
    }

    #[tokio::test]
    async fn should_return_429() {
        // Arrange
        let server = MockServer::start();
        let mock1 = server.mock(|when, then| {
            when.method("GET").path("/api/v2/ability/42");
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
            then.status(429);
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
        assert_eq!(res.status(), 429, "GET return 429 if clients calls quickly");
    }

    #[tokio::test]
    async fn should_return_404_if_not_exist() {
        // Arrange
        let server = MockServer::start();
        let mock1 = server.mock(|when, then| {
            when.method("GET").path("/api/v2/ability/42");
            then.status(404);
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
        assert_eq!(res.status(), 404, "GET return 404 if poke not exist");
    }

    #[tokio::test]
    async fn should_return_404_if_no_description() {
        // Arrange
        let server = MockServer::start();
        let mock1 = server.mock(|when, then| {
            when.method("GET").path("/api/v2/ability/42");
            then.status(200)
                .json_body(json!({
                    "effect_entries": [
                        {
                            "effect": "Mit jedem Treffer besteht eine 10% Chance das Ziel zurückschrecken zu lassen.",
                            "language": { "name": "de" }
                        },
                    ],
                }));
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
        assert_eq!(res.status(), 404, "GET return 404 if poke descr. not exist");
    }
}
