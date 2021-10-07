use pokeapi_types::Pokemon;

pub async fn get_pokemon(id: u32) -> Pokemon {
    reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{id}/", id = id))
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_pokemon() {
        assert_eq!(get_pokemon(1).await.name, "bulbasaur");
        assert_eq!(get_pokemon(2).await.name, "ivysaur");
        assert_eq!(get_pokemon(3).await.name, "venusaur");
    }
}
