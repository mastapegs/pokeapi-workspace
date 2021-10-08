//! # pokeapi-types
//!
//! `pokeapi-types` is a collection of structs used to aid in the creation of
//! data from the [PokeAPI](https://pokeapi.co/) service.

mod pokemon;
pub use pokemon::*;

mod ability;
pub use ability::*;

mod common;
pub use common::*;
