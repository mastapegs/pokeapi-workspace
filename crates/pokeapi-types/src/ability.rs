use serde::{Deserialize, Serialize};

/// This is the JSON struct for the endpoint /ability/{id}
#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
    id: u32,
    is_main_series: bool,
    name: String,
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
        println!("{:#?}", ability);
    }
}
