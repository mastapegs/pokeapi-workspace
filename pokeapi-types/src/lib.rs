use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMoveVersion {
    pub move_learn_method: NamedAPIResource,
    pub version_group: NamedAPIResource,
    pub level_learned_at: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMove {
    pub r#move: NamedAPIResource,
    pub version_group_details: Vec<PokemonMoveVersion>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonHeldItemVersion {
    pub version: NamedAPIResource,
    pub rarity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonHeldItem {
    pub item: NamedAPIResource,
    pub version_details: Vec<PokemonHeldItemVersion>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionGameIndex {
    pub game_index: u32,
    pub version: NamedAPIResource,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NamedAPIResource {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonAbility {
    pub ability: NamedAPIResource,
    pub is_hidden: bool,
    pub slot: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    pub abilities: Vec<PokemonAbility>,
    pub base_experience: u32,
    pub forms: Vec<NamedAPIResource>,
    pub game_indices: Vec<VersionGameIndex>,
    pub height: u32,
    pub held_items: Vec<PokemonHeldItem>,
    pub id: u32,
    pub is_default: bool,
    pub location_area_encounters: String,
    pub moves: Vec<PokemonMove>,
    pub name: String,
    pub order: u32,
    pub species: NamedAPIResource,
    pub weight: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let json: Pokemon = response.json().await.unwrap();
        assert_eq!(json.name, "bulbasaur");
        println!("{:#?}", json);
    }
}
