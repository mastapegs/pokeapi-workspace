//! # pokeapi-getters
//!
//! `pokeapi-getters` is a collection of async getter functions that assist
//! in the retrieval of data from the [PokeAPI](https://pokeapi.co/).

mod get_pokemon;
pub use get_pokemon::get_pokemon;

mod get_ability;
pub use get_ability::get_ability;
