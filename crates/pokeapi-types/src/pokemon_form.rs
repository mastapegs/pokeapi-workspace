use crate::NamedAPIResource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonFormSprites {
    pub front_default: String,
    pub front_shiny: String,
    pub back_default: String,
    pub back_shiny: String,
}

/// This is the JSON struct for the endpoint /pokemon-form/{id}
#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonForm {
    pub form_name: String,
    pub form_order: usize,
    pub id: usize,
    pub is_battle_only: bool,
    pub is_default: bool,
    pub is_mega: bool,
    pub name: String,
    pub order: usize,
    pub pokemon: NamedAPIResource,
    pub sprites: PokemonFormSprites,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_serialize_to_pokemon() {
        let response = reqwest::get("https://pokeapi.co/api/v2/pokemon-form/1/")
            .await
            .unwrap();
        assert!(&response.status().is_success());
        let form: PokemonForm = response.json().await.unwrap();
        assert_eq!(form.name, "bulbasaur");
    }
}
