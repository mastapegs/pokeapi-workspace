use crate::{Name, NamedAPIResource};
use serde::{Deserialize, Serialize};

/// This is the JSON struct for the endpoint /item/{id}
///
/// # Examples
///
/// ```
/// use pokeapi_types::Item;
///
/// async fn get_poison_heal() {
///     let response = reqwest::get("https://pokeapi.co/api/v2/item/1/")
///         .await
///         .unwrap();
///     let master_ball: Item = response.json().await.unwrap();
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_serialize_to_ability() {
        let response = reqwest::get("https://pokeapi.co/api/v2/item/1/")
            .await
            .unwrap();
        assert!(&response.status().is_success());
        let item: Item = response.json().await.unwrap();
        assert_eq!(item.name, "master-ball");
    }
}
