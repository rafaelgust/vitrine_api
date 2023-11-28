use rocket::http::uri::Origin;
use rocket::response::{Redirect, status::Accepted, status::NotFound};
use rocket::serde::json::Json;

use crate::models::{Brand, Department, SubDepartment, Product};
use crate::ops::brand_ops::{self, BrandResult};
use crate::ops::department_ops::{self, DepartmentResult};
use crate::ops::sub_department_ops::{self, SubDepartmentResult};

use crate::args::{BrandCommand, BrandSubcommand, DepartmentCommand, DepartmentSubcommand, SubDepartmentCommand, SubDepartmentSubcommand};
use crate::args::{GetEntity, CreateWithNameEntity, DeleteEntity};
use crate::args::{
    UpdateBrand as UpdateBrandArgs, 
    CreateDepartment, UpdateDepartment as UpdateDepartmentArgs, 
    CreateSubDepartment, UpdateSubDepartment as UpdateSubDepartmentArgs
};

const BRAND_NOT_FOUND: &str = "Unable to find brand";
const DEPARTMENT_NOT_FOUND: &str = "Unable to find department";
const FETCH_ERROR: &str = "An error occurred while fetching data";
const UNEXPECTED_RESULT: &str = "Unexpected result";

pub const URI_BRAND : Origin<'static> = uri!("/brand");
pub const URI_DEPARTMENT : Origin<'static> = uri!("/department");
pub const URI_PRODUCT : Origin<'static> = uri!("/products");

#[get("/")]
pub fn get_index() -> Redirect {
    Redirect::to(URI_PRODUCT)
}

//Brand
#[post("/", data = "<new_brand>", format = "application/json")]
pub fn new_brand(new_brand: Json<CreateWithNameEntity>) -> Result<Accepted<String>, NotFound<String>> {

    let brand = CreateWithNameEntity {
        name: new_brand.name.trim().to_string(),
    };
    
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Create(brand),
    });
    
    match result {
        Ok(BrandResult::Message(_)) => Ok(Accepted(format!("Brand '{}' was created", new_brand.name.trim().to_string()))),
        Ok(_) => Err(NotFound(format!("Unable to find brand"))),
        Err(_) => Err(NotFound(format!("An error occurred while fetching brand"))),
    }
}

#[put("/", data = "<brand>", format = "application/json")]
pub fn update_brand(brand: Json<UpdateBrandArgs>) -> Result<Accepted<Json<Brand>>, NotFound<String>> {
    
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
pub fn delete_brand(brand: Json<DeleteEntity>) ->  Result<Accepted<String>, NotFound<String>> {

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

//Departments ~ SubDepartment

#[post("/", data = "<new_department>", format = "application/json")]
pub fn new_department(new_department: Json<CreateDepartment>) -> Result<Accepted<String>, NotFound<String>> {

    let department = CreateDepartment {
        name: new_department.name.trim().to_string(),
        color: "#FFFFFF".to_string(),
    };
    
    let result = department_ops::handle_department_command(DepartmentCommand {
        command: DepartmentSubcommand::Create(department),
    });
    
    match result {
        Ok(DepartmentResult::Message(_)) => Ok(Accepted(format!("Department '{}' was created", new_department.name.trim().to_string()))),
        Ok(_) => Err(NotFound(format!("Unable to find department"))),
        Err(_) => Err(NotFound(format!("An error occurred while fetching department"))),
    }
}

#[put("/", data = "<department>", format = "application/json")]
pub fn update_department(department: Json<UpdateDepartmentArgs>) -> Result<Accepted<Json<Department>>, NotFound<String>> {
   
    let department = UpdateDepartmentArgs {
        id: department.id,
        name: department.name.to_string(),
        color: department.color.to_string(),
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
pub fn delete_department(department: Json<DeleteEntity>) ->  Result<Accepted<String>, NotFound<String>> {

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

#[post("/subdepartment", data = "<new_sub_department>", format = "application/json")]
pub fn new_sub_department(new_sub_department: Json<CreateSubDepartment>) -> Result<Accepted<String>, NotFound<String>> {

    let sub_department = CreateSubDepartment {
        name: new_sub_department.name.trim().to_string(),
        department_id: new_sub_department.department_id,
    };
    
    let result = sub_department_ops::handle_sub_department_command(SubDepartmentCommand {
        command: SubDepartmentSubcommand::Create(sub_department),
    });
    
    match result {
        Ok(SubDepartmentResult::Message(_)) => Ok(Accepted(format!("SubDepartment '{}' was created", new_sub_department.name.trim().to_string()))),
        Ok(_) => Err(NotFound(format!("Unable to find sub department"))),
        Err(_) => Err(NotFound(format!("An error occurred while fetching sub department"))),
    }
}

#[put("/subdepartment", data = "<sub_department>", format = "application/json")]
pub fn update_sub_department(sub_department: Json<UpdateSubDepartmentArgs>) -> Result<Accepted<Json<SubDepartment>>, NotFound<String>> {
   
    let sub_department = UpdateSubDepartmentArgs {
        id: sub_department.id,
        name: sub_department.name.trim().to_string(),
        department_id: sub_department.id,
    };
    
    let result = sub_department_ops::handle_sub_department_command(SubDepartmentCommand {
        command: SubDepartmentSubcommand::Update(sub_department),
    });

    match result {
        Ok(SubDepartmentResult::SubDepartment(Some(result))) => Ok(Accepted(Json(result))),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}

#[delete("/subdepartment", data = "<sub_department>", format = "application/json")]
pub fn delete_sub_department(sub_department: Json<DeleteEntity>) ->  Result<Accepted<String>, NotFound<String>> {

    let result = sub_department_ops::handle_sub_department_command(SubDepartmentCommand {
        command: SubDepartmentSubcommand::Delete(DeleteEntity {
            id: sub_department.id,
        }),
    });

    match result {
        Ok(SubDepartmentResult::Message(msg)) => Ok(Accepted(msg)),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}

#[get("/subdepartment", format = "application/json")]
pub fn get_all_sub_departments() -> Result<Json<Vec<SubDepartment>>, NotFound<String>> {
    
    let result = sub_department_ops::handle_sub_department_command(SubDepartmentCommand {
        command: SubDepartmentSubcommand::ShowAll,
    });

    match result {
        Ok(SubDepartmentResult::SubDepartments(department)) => Ok(Json(department)),
        Ok(_) => Err(NotFound(DEPARTMENT_NOT_FOUND.to_string())),
        Err(_) => Err(NotFound(FETCH_ERROR.to_string())),
    }
}

#[get("/<sub_department_name>", format = "application/json")]
pub fn get_sub_department(sub_department_name: String) ->  Result<Json<SubDepartment>, NotFound<String>> {
    
    let result = sub_department_ops::handle_sub_department_command(SubDepartmentCommand {
        command: SubDepartmentSubcommand::Show(GetEntity {
            name: sub_department_name,
        }),
    });

    match result {
        Ok(SubDepartmentResult::SubDepartment(Some(department))) => Ok(Json(department)),
        Ok(_) => Err(NotFound(DEPARTMENT_NOT_FOUND.to_string())),
        Err(_) => Err(NotFound(FETCH_ERROR.to_string())),
    }
}

//Products

#[get("/all")]
pub fn get_products() -> Json<Vec<Product>> {
    Json(vec![Product {
        id: 1,
        url_name: "product_1".into(),
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