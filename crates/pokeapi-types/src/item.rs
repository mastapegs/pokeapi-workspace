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
    pub attributes: Vec<NamedAPIResource>,
    pub cost: usize,
    // pub fling_effect: NamedAPIResource,
    // pub fling_power: usize,
    pub id: usize,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_deserialize_to_item_struct() {
        let response = reqwest::get("https://pokeapi.co/api/v2/item/1/")
            .await
            .unwrap();
        assert!(&response.status().is_success());
        let item: Item = response.json().await.unwrap();
        assert_eq!(item.name, "master-ball");
    }
}
