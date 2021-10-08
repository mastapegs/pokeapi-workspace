use pokeapi_getters::get_pokemon;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let pokemon_id = &args[1].parse::<u32>().unwrap();
    let pokemon = get_pokemon(*pokemon_id).await;

    println!("The pokemon retrieved is: {name}", name = pokemon.name);
}
