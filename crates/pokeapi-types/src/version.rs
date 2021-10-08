use crate::{Name, NamedAPIResource};
use serde::{Deserialize, Serialize};

/// This is the JSON struct for the endpoint /version/{id}
///
/// # Examples
///
/// ```
/// use pokeapi_types::Version;
///
/// async fn get_red_version() {
///     let response = reqwest::get("https://pokeapi.co/api/v2/version/90/")
///         .await
///         .unwrap();
///     let red: Version = response.json().await.unwrap();
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub id: usize,
    pub name: String,
    pub names: Vec<Name>,
    pub version_group: NamedAPIResource,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_serialize_to_ability() {
        let response = reqwest::get("https://pokeapi.co/api/v2/version/1/")
            .await
            .unwrap();
        assert!(&response.status().is_success());
        let version: Version = response.json().await.unwrap();
        assert_eq!(version.name, "red");
    }
}
