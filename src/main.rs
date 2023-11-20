use rocket::http::uri::Origin;
use rocket::response::Redirect;

const URI_PRODUCT : Origin<'static> = uri!("/product");

#[macro_use] 
extern crate rocket;

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
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount(URI_PRODUCT, routes![product])
    .mount("/department", routes![department])
}