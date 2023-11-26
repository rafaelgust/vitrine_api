use rocket::http::uri::Origin;
use rocket::response::{Redirect, status::Accepted, status::NotFound};
use rocket::serde::json::Json;

use crate::models::*;
use crate::ops::brand_ops::{self, BrandResult};
use crate::ops::department_ops::{self, DepartmentResult};

use crate::args::{BrandCommand, BrandSubcommand, DepartmentCommand, DepartmentSubcommand};
use crate::args::{GetEntity, CreateWithNameEntity, DeleteEntity, UpdateBrand as UpdateBrandArgs, UpdateDepartment as UpdateDepartmentArgs};

const BRAND_NOT_FOUND: &str = "Unable to find brand";
const DEPARTMENT_NOT_FOUND: &str = "Unable to find department";
const FETCH_ERROR: &str = "An error occurred while fetching data";
const UNEXPECTED_RESULT: &str = "Unexpected result";

#[get("/")]
pub fn get_index() -> Redirect {
    Redirect::to(URI_PRODUCT)
}

pub const URI_BRAND : Origin<'static> = uri!("/brand");

#[post("/", data = "<new_brand>", format = "application/json")]
pub fn new_brand(new_brand: Json<NewBrand<'_>>) -> Result<Accepted<String>, NotFound<String>> {

    let brand = CreateWithNameEntity {
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
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
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
        Ok(_) => Err(NotFound(BRAND_NOT_FOUND.to_string())),
        Err(_) => Err(NotFound(FETCH_ERROR.to_string())),
    }
}

#[get("/<brand_name>", format = "application/json")]
pub fn get_brand(brand_name: String) ->  Result<Json<Brand>, NotFound<String>> {
    
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Show(GetEntity {
            name: brand_name,
        }),
    });

    match result {
        Ok(BrandResult::Brand(Some(brand))) => Ok(Json(brand)),
        Ok(_) => Err(NotFound(BRAND_NOT_FOUND.to_string())),
        Err(_) => Err(NotFound(FETCH_ERROR.to_string())),
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
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}

pub const URI_DEPARTMENT : Origin<'static> = uri!("/department");

#[post("/", data = "<new_department>", format = "application/json")]
pub fn new_department(new_department: Json<NewDepartment<'_>>) -> Result<Accepted<String>, NotFound<String>> {

    let department = CreateWithNameEntity {
        name: new_department.name.to_string(),
    };
    
    let result = department_ops::handle_department_command(DepartmentCommand {
        command: DepartmentSubcommand::Create(department),
    });
    
    match result {
        Ok(DepartmentResult::Message(_)) => Ok(Accepted(format!("Department was created"))),
        Ok(_) => Err(NotFound(format!("Unable to find department"))),
        Err(_) => Err(NotFound(format!("An error occurred while fetching department"))),
    }
}

#[put("/", data = "<department>", format = "application/json")]
pub fn update_department(department: Json<UpdateDepartment>) -> Result<Accepted<Json<Department>>, NotFound<String>> {
    
    let department = UpdateDepartmentArgs {
        id: department.id,
        name: department.name.to_string(),
    };
    
    let result = department_ops::handle_department_command(DepartmentCommand {
        command: DepartmentSubcommand::Update(department),
    });

    match result {
        Ok(DepartmentResult::Department(Some(department))) => Ok(Accepted(Json(department))),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}

#[get("/", format = "application/json")]
pub fn get_all_departments() -> Result<Json<Vec<Department>>, NotFound<String>> {
    
    let result = department_ops::handle_department_command(DepartmentCommand {
        command: DepartmentSubcommand::ShowAll,
    });

    match result {
        Ok(DepartmentResult::Departments(department)) => Ok(Json(department)),
        Ok(_) => Err(NotFound(DEPARTMENT_NOT_FOUND.to_string())),
        Err(_) => Err(NotFound(FETCH_ERROR.to_string())),
    }
}

#[get("/<department_name>", format = "application/json")]
pub fn get_department(department_name: String) ->  Result<Json<Department>, NotFound<String>> {
    
    let result = department_ops::handle_department_command(DepartmentCommand {
        command: DepartmentSubcommand::Show(GetEntity {
            name: department_name,
        }),
    });

    match result {
        Ok(DepartmentResult::Department(Some(department))) => Ok(Json(department)),
        Ok(_) => Err(NotFound(DEPARTMENT_NOT_FOUND.to_string())),
        Err(_) => Err(NotFound(FETCH_ERROR.to_string())),
    }
}

#[delete("/", data = "<department>", format = "application/json")]
pub fn delete_department(department: Json<RemoveDepartment>) ->  Result<Accepted<String>, NotFound<String>> {

    let result = department_ops::handle_department_command(DepartmentCommand {
        command: DepartmentSubcommand::Delete(DeleteEntity {
            id: department.id,
        }),
    });

    match result {
        Ok(DepartmentResult::Message(msg)) => Ok(Accepted(msg)),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
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