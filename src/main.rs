#![allow(unused_imports)]

#[macro_use] 
extern crate rocket;


use rocket::serde::json::Json;
use rocket::http::uri::Origin;
use rocket::response::Redirect;


const URI_PRODUCT : Origin<'static> = uri!("/product");


#[get("/")]
fn index() -> Redirect {
    Redirect::to(URI_PRODUCT)
}

#[get("/<name>")]
fn product(name: &str) -> String {
    format!("Product {}", name)
}

#[get("/<name>")]
fn department(name: &str) -> String {
    format!("Department {}", name)
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount(URI_PRODUCT, routes![product])
    .mount("/department", routes![department])
}