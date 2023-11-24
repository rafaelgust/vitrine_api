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

   /*  /// Create, update, delete or show departments
    Department(DepartmentCommand),

    /// Create, update, delete or show subdepartments
    SubDepartment(SubDepartmentCommand),

    /// Create, update, delete or show products
    Product(ProductCommand), */
}

#[derive(Debug, Args)]
pub struct BrandCommand {
    #[clap(subcommand)]
    pub command: BrandSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum BrandSubcommand {
    /// Show brand by name
    Show(GetBrand),

    /// Create a new brand
    Create(CreateBrand),

    /// Update an existing brand
    Update(UpdateBrand),

    /// Delete a brand
    Delete(DeleteEntity),

    /// Show all brands
    ShowAll,
}

#[derive(Debug, Args)]
pub struct GetBrand {
    /// The name of the brand to find
    pub name: String,
}

#[derive(Debug, Args)]
pub struct CreateBrand {
    /// The name of the brand
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
pub struct DeleteEntity {
    /// The id of the entity to delete
    pub id: i32,
}
