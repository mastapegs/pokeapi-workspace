#[macro_use] extern crate rocket;

use pokeapi_getters::get_pokemon;
use pokeapi_types::Pokemon;
use rocket::serde::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!! From a Rust server!!!"
}

#[get("/pokemon/<id>")]
async fn pokemon(id: &str) -> Json<Pokemon> {
    Json(get_pokemon(id.parse::<u32>().unwrap()).await)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, pokemon])
}