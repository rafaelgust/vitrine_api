use rocket::http::uri::Origin;
use rocket::response::{Redirect, status::Accepted, status::NotFound};
use rocket::serde::json::Json;

use crate::models::*;

use crate::args::{BrandCommand, BrandSubcommand, DeleteEntity};
use crate::args::{GetBrand, CreateBrand, UpdateBrand as UpdateBrandArgs};
use crate::ops::brand_ops::{self, BrandResult};


#[get("/")]
pub fn get_index() -> Redirect {
    Redirect::to(URI_PRODUCT)
}

pub const URI_BRAND : Origin<'static> = uri!("/brand");

#[post("/", data = "<new_brand>", format = "application/json")]
pub fn new_brand(new_brand: Json<NewBrand<'_>>) -> Result<Accepted<String>, NotFound<String>> {

    let brand = CreateBrand {
        name: new_brand.name.to_string(),
    };
    
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Create(brand),
    });
    
    match result {
        Ok(BrandResult::Message(_)) => Ok(Accepted(format!("Brand was created"))),
        Ok(_) => Err(NotFound(format!("Unable to find brand"))),
        Err(_) => Err(NotFound(format!("An error occurred while fetching brand"))),
    }
}

#[put("/", data = "<brand>", format = "application/json")]
pub fn update_brand(brand: Json<UpdateBrand>) -> Result<Accepted<Json<Brand>>, NotFound<String>> {
    
    let brand = UpdateBrandArgs {
        id: brand.id,
        name: brand.name.to_string(),
    };
    
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Update(brand),
    });

    match result {
        Ok(BrandResult::Brand(Some(brand))) => Ok(Accepted(Json(brand))),
        Ok(_) => Err(NotFound(format!("Unexpected result"))),
        Err(err) => Err(NotFound(err.to_string())),
    }
}

#[get("/", format = "application/json")]
pub fn get_all_brands() -> Result<Json<Vec<Brand>>, NotFound<String>> {
    
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::ShowAll,
    });

    match result {
        Ok(BrandResult::Brands(brand)) => Ok(Json(brand)),
        Ok(_) => Err(NotFound(format!("Unable to find brand"))),
        Err(_) => Err(NotFound(format!("An error occurred while fetching brand"))),
    }
}

#[get("/<brand_name>", format = "application/json")]
pub fn get_brand(brand_name: String) ->  Result<Json<Brand>, NotFound<String>> {
    
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Show(GetBrand {
            name: brand_name,
        }),
    });

    match result {
        Ok(BrandResult::Brand(Some(brand))) => Ok(Json(brand)),
        Ok(_) => Err(NotFound(format!("Unable to find brand"))),
        Err(_) => Err(NotFound(format!("An error occurred while fetching brand"))),
    }
}

#[delete("/", data = "<brand>", format = "application/json")]
pub fn delete_brand(brand: Json<RemoveBrand>) ->  Result<Accepted<String>, NotFound<String>> {

    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Delete(DeleteEntity {
            id: brand.id,
        }),
    });

    match result {
        Ok(BrandResult::Message(msg)) => Ok(Accepted(msg)),
        Ok(_) => Err(NotFound(format!("Unexpected"))),
        Err(_) => Err(NotFound(format!("An error occurred while was deleting brand"))),
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
    Json(vec![Product {
        id: 1,
        name: "Product 1".into(),
        price: Some(10.0),
        description: "Description".into(),
        images: vec![Some("image1".into()), Some("image2".into())],
        brand_id: Some(1),
        department_id: Some(1),
    }])
}

#[get("/product/<name>")]
pub fn get_product(name: Option<String>) -> String {
    match name {
        Some(name) => format!("Product {}", name),
        None => "Product not found".into()
    }
}