use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    pub name: String,
    pub language: NamedAPIResource,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NamedAPIResource {
    pub name: String,
    pub url: String,
}
