use pokeapi_types::Pokemon;

/// Get the pokemon with a given id
///
/// # Example
///
/// ```
/// use pokeapi_getters::get_pokemon;
///
/// async fn using_get_pokemon() {
///     let bulbasaur = get_pokemon(1).await;
/// }
/// ```
///
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
    async fn should_get_correct_pokemon() {
        assert_eq!(get_pokemon(1).await.name, "bulbasaur");
        assert_eq!(get_pokemon(2).await.name, "ivysaur");
        assert_eq!(get_pokemon(3).await.name, "venusaur");
    }
}
