use pokeapi_types::Ability;

/// Get the ability with a given id
///
/// # Example
///
/// ```
/// use pokeapi_getters::get_ability;
///
/// async fn using_get_ability() {
///     let stench = get_ability(1).await;
/// }
/// ```
///
pub async fn get_ability(id: u32) -> Ability {
    reqwest::get(format!("https://pokeapi.co/api/v2/ability/{id}/", id = id))
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
        assert_eq!(get_ability(1).await.name, "stench");
        assert_eq!(get_ability(2).await.name, "drizzle");
        assert_eq!(get_ability(3).await.name, "speed-boost");
    }
}
