use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AbilityDetails {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
    pub ability: AbilityDetails,
    pub is_hidden: bool,
    pub slot: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    pub abilities: Vec<Ability>,
    pub base_experience: u32,
    pub height: u32,
    pub id: u32,
    pub is_default: bool,
    pub location_area_encounters: String,
    pub name: String,
    pub order: u32,
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
