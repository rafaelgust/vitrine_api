use rocket::http::uri::Origin;
use rocket::response::Redirect;
use diesel::RunQueryDsl;

use crate::db::establish_connection;
use crate::models::Product;

pub const URI_PRODUCT : Origin<'static> = uri!("/products");

#[get("/")]
pub fn get_index() -> Redirect {
    Redirect::to(URI_PRODUCT)
}

#[get("/all")]
pub fn get_products() {
    use crate::schema::products::dsl::*;

    let mut connection = establish_connection();
    let results = products
    .load::<Product>(&mut connection)
        .unwrap();

    println!("Displaying {} users", results.len());
    for prod in results {
        println!("{:?}", prod);
    }
}

#[get("/product/<name>")]
pub fn get_product(name: &str) -> String {
    format!("Product {}", name)
}

#[get("/<name>")]
pub fn get_department(name: &str) -> String {
    format!("Department {}", name)
}