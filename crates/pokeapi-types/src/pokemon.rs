use crate::NamedAPIResource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonType {
    pub slot: u8,
    pub r#type: NamedAPIResource,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonStat {
    pub stat: NamedAPIResource,
    pub effort: u32,
    pub base_stat: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonSprites {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny_female: Option<String>,
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny_female: Option<String>,
}

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
pub struct PokemonAbility {
    pub ability: NamedAPIResource,
    pub is_hidden: bool,
    pub slot: u32,
}

/// This is the JSON struct for the endpoint /pokemon/{id}
///
/// # Examples
///
/// ```
/// use pokeapi_types::Pokemon;
///
/// async fn getBulbasaur() {
///     let response = reqwest::get("https://pokeapi.co/api/v2/pokemon/1/")
///         .await
///         .unwrap();
///     let bulbasaur: Pokemon = response.json().await.unwrap();
/// }
/// ```
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
    pub sprites: PokemonSprites,
    pub stats: Vec<PokemonStat>,
    pub types: Vec<PokemonType>,
    pub weight: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_serialize_to_pokemon() {
        let response = reqwest::get("https://pokeapi.co/api/v2/pokemon/1/")
            .await
            .unwrap();
        assert!(&response.status().is_success());
        let pokemon: Pokemon = response.json().await.unwrap();
        assert_eq!(pokemon.name, "bulbasaur");
    }
}
