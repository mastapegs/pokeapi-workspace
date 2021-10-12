#[macro_use] extern crate rocket;

use pokeapi_getters::get_pokemon;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!! From a Rust server!!!"
}

#[get("/pokemon/<id>")]
async fn pokemon(id: &str) -> String {
    get_pokemon(id.parse::<u32>().unwrap()).await.name
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, pokemon])
}