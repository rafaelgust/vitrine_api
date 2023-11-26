#[macro_use]
extern crate rocket;
extern crate diesel;

mod db;
mod router;
mod models;
mod schema;

mod ops;
mod args;

use rocket::Rocket;
use rocket::Build;

fn rocket() -> Rocket<Build> {
    rocket::build()
    .mount("/", routes![router::get_index])
    .mount(router::URI_PRODUCT, routes![router::get_products, router::get_product])
    .mount(router::URI_DEPARTMENT, routes![router::new_department, router::update_department, router::get_department, router::get_all_departments, router::delete_department])
    .mount(router::URI_BRAND, routes![router::get_brand, router::update_brand, router::get_all_brands, router::new_brand, router::delete_brand])
}

#[rocket::main]
async fn main() {
    rocket().launch().await.expect("Failed to launch Rocket");
}