use clap::{
    Args, 
    Parser, 
    Subcommand
};

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
}

#[derive(Debug, Args)]
pub struct EntityCommand<T: Subcommand> {
    #[clap(subcommand)]
    pub command: T,
}

pub type BrandCommand = EntityCommand<BrandSubcommand>;
pub type DepartmentCommand = EntityCommand<DepartmentSubcommand>;

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
    Create(CreateWithNameEntity),

    /// Update an existing department
    Update(UpdateDepartment),

    /// Delete a department
    Delete(DeleteEntity),

    /// Show all departments
    ShowAll,
}

#[derive(Debug, Args)]
pub struct GetEntity {
    /// The name of the entity to find
    pub name: String,
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    /// The id of the entity to delete
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct CreateWithNameEntity {
    /// The name of the entity
    pub name: String,
}

#[derive(Debug, Args)]
pub struct UpdateBrand {
    /// The id of the brand to update
    pub id: i32,

    /// The name of the brand
    pub name: String,
}

#[derive(Debug, Args)]
pub struct UpdateDepartment {
    /// The id of the department to update
    pub id: i32,

    /// The name of the department
    pub name: String,
}