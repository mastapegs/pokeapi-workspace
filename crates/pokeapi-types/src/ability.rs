use crate::NamedAPIResource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Effect {
    pub effect: String,
    pub language: NamedAPIResource,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbilityEffectChange {
    pub effect_entries: Vec<Effect>,
    pub version_group: NamedAPIResource,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerboseEffect {
    pub effect: String,
    pub language: NamedAPIResource,
    pub short_effect: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    pub name: String,
    pub language: NamedAPIResource,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbilityFlavorText {
    pub flavor_text: String,
    pub language: NamedAPIResource,
    pub version_group: NamedAPIResource,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbilityPokemon {
    pub is_hidden: bool,
    pub slot: u32,
    pub pokemon: NamedAPIResource,
}

/// This is the JSON struct for the endpoint /ability/{id}
#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
    pub generation: NamedAPIResource,
    pub id: u32,
    pub is_main_series: bool,
    pub name: String,
    pub names: Vec<Name>,
    pub effect_entries: Vec<VerboseEffect>,
    pub effect_changes: Vec<AbilityEffectChange>,
    pub flavor_text_entries: Vec<AbilityFlavorText>,
    pub pokemon: Vec<AbilityPokemon>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_serialize_to_ability() {
        let response = reqwest::get("https://pokeapi.co/api/v2/ability/1/")
            .await
            .unwrap();
        assert!(&response.status().is_success());
        let ability: Ability = response.json().await.unwrap();
        assert_eq!(ability.name, "stench");
    }
}
