#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn async_test_works() {
        let response = reqwest::get("https://pokeapi.co/api/v2/pokemon/1/")
            .await
            .unwrap();
        assert!(&response.status().is_success());
        let json: serde_json::Value = response.json().await.unwrap();
        assert_eq!(json["name"], "bulbasaur");
    }
}
