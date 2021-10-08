use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NamedAPIResource {
    pub name: String,
    pub url: String,
}
