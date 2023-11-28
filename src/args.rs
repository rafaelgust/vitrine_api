use clap::{
    Args, 
    Parser, 
    Subcommand
};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct ApiArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, update, delete or show brands
    Brand(BrandCommand),

    /// Create, update, delete or show departments
    Department(DepartmentCommand),

    /// Create, update, delete or show sub departments
    SubDepartment(SubDepartmentCommand),
}

#[derive(Debug, Args)]
pub struct EntityCommand<T: Subcommand> {
    #[clap(subcommand)]
    pub command: T,
}

pub type BrandCommand = EntityCommand<BrandSubcommand>;
pub type DepartmentCommand = EntityCommand<DepartmentSubcommand>;
pub type SubDepartmentCommand = EntityCommand<SubDepartmentSubcommand>;

#[derive(Debug, Subcommand)]
pub enum BrandSubcommand {
    /// Show brand by name
    Show(GetEntity),

    /// Create a new brand
    Create(CreateWithNameEntity),

    /// Update an existing brand
    Update(UpdateBrand),

    /// Delete a brand
    Delete(DeleteEntity),

    /// Show all brands
    ShowAll,
}

#[derive(Debug, Subcommand)]
pub enum DepartmentSubcommand {
    /// Show department by name
    Show(GetEntity),

    /// Create a new department
    Create(CreateDepartment),

    /// Update an existing department
    Update(UpdateDepartment),

    /// Delete a department
    Delete(DeleteEntity),

    /// Show all departments
    ShowAll,
}

#[derive(Debug, Subcommand)]
pub enum SubDepartmentSubcommand {
    /// Show department by name
    Show(GetEntity),

    /// Create a new sub department
    Create(CreateSubDepartment),

    /// Update an existing sub department
    Update(UpdateSubDepartment),

    /// Delete a sub department
    Delete(DeleteEntity),

    /// Show all sub departments
    ShowAll,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct GetEntity {
    /// The name of the entity to find
    pub name: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct DeleteEntity {
    /// The id of the entity to delete
    pub id: i32,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CreateWithNameEntity {
    /// The name of the entity
    pub name: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UpdateBrand {
    /// The id of the brand to update
    pub id: i32,

    /// The name of the brand
    pub name: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CreateDepartment {
    /// The name of the department
    pub name: String,

    /// The color of the department
    pub color: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UpdateDepartment {
    /// The id of the department to update
    pub id: i32,

    /// The name of the department
    pub name: String,

    /// The color of the department
    pub color: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CreateSubDepartment {
    /// The name of the sub department
    pub name: String,

    /// The color of the sub department
    pub department_id: i32,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UpdateSubDepartment {
    /// The id of the department to update
    pub id: i32,

    /// The name of the sub department
    pub name: String,

    /// The color of the sub department
    pub department_id: i32,
}