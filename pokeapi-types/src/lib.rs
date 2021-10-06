use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    base_experience: u32,
    height: u32,
    id: u32,
    is_default: bool,
    location_area_encounters: String,
    name: String,
    order: u32,
    weight: u32,
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
