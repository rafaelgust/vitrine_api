#[macro_use]
extern crate rocket;
extern crate diesel;

mod db;
mod router;
mod models;
mod schema;

use rocket::Rocket;
use rocket::Build;

fn rocket() -> Rocket<Build> {
    rocket::build()
    .mount("/", routes![router::get_index])
    .mount(router::URI_PRODUCT, routes![router::get_products])
    .mount(router::URI_PRODUCT, routes![router::get_product])
    .mount("/department", routes![router::get_department])
}

#[rocket::main]
async fn main() {
    rocket().launch().await.expect("Failed to launch Rocket");
}