use rocket::http::uri::Origin;
use rocket::response::{Redirect, status::Created, status::NotFound};
use rocket::serde::json::Json;
use diesel::prelude::*;

use crate::db::establish_connection;
use crate::models::*;

#[get("/")]
pub fn get_index() -> Redirect {
    Redirect::to(URI_PRODUCT)
}

pub const URI_BRAND : Origin<'static> = uri!("/brand");

#[post("/", data = "<brand>")]
pub fn new_brand(brand: String) -> Result<String, NotFound<String>> {
   /*  use crate::schema::brands::dsl::*;

    let connection = &mut establish_connection();

    let new_brand = NewBrand{ name: brand };

    diesel::insert_into(brands)
    .values(new_brand)
    .execute(connection)
    .optional();
    
    match brand {
        Ok(_) => Ok(format!("Unable to find brand")),
        Err(_) => Err(NotFound(format!("An error occurred while fetching brand"))),
    } */
    Ok(format!("Unable to find brand"))
}

#[get("/<brandname>")]
pub fn get_brand(brandname: String) ->  Result<Json<Vec<Brand>>, NotFound<String>> {
    use crate::schema::brands::dsl::*;

    let connection = &mut establish_connection();

    let brand = brands
    .filter(name.eq(brandname))
    .select(Brand::as_select())
    .first(connection)
    .optional();

    match brand {
        Ok(Some(brand)) => {
            println!("Brand with name: {} found", brand.name);
            Ok(Json(vec![brand]))
        },
        Ok(None) => Err(NotFound(format!("Unable to find brand"))),
        Err(_) => Err(NotFound(format!("An error occurred while fetching brand"))),
    }
}

pub const URI_DEPARTMENT : Origin<'static> = uri!("/department");

#[get("/<name>")]
pub fn get_department(name: Option<String>) -> String {
    match name {
        Some(name) => format!("Department {}", name),
        None => "Department not found".into()
    }
}

pub const URI_PRODUCT : Origin<'static> = uri!("/products");

#[get("/all")]
pub fn get_products() -> Json<Vec<Product>> {
    use crate::schema::products::dsl::*;

    let connection = &mut establish_connection();
    products.load::<Product>(connection).map(Json).expect("Error loading products")
}

#[get("/product/<name>")]
pub fn get_product(name: Option<String>) -> String {
    match name {
        Some(name) => format!("Product {}", name),
        None => "Product not found".into()
    }
}