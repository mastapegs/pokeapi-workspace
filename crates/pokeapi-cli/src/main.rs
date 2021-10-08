use pokeapi_getters::get_pokemon;

#[tokio::main]
async fn main() {
    let pokemon = get_pokemon(13).await;
    println!("The pokemon retrieved is: {name}", name = pokemon.name);
}
