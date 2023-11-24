use crate::args::{
    BrandSubcommand, 
    BrandCommand, 
    GetBrand, 
    CreateBrand, 
    UpdateBrand, 
    DeleteEntity
};
use crate::db::establish_connection;
use crate::models::{NewBrand, Brand as DBBrand};
use diesel::prelude::*;
use diesel::result::Error;

pub enum BrandResult {
    Brand(Option<DBBrand>),
    Brands(Vec<DBBrand>),
    Message(String),
}

pub fn handle_brand_command(brand: BrandCommand) -> Result<BrandResult, Error> {
    let command = brand.command;
    match command {
        BrandSubcommand::Show(brand) => {
            show_brand_by_name(brand).map(BrandResult::Brand)
        }
        BrandSubcommand::Create(brand) => {
            create_brand(brand).map(BrandResult::Message)
        }
        BrandSubcommand::Update(brand) => {
            update_brand(brand).map(BrandResult::Brand)
        }
        BrandSubcommand::Delete(delete_entity) => {
            delete_brand(delete_entity).map(BrandResult::Message)
        }
        BrandSubcommand::ShowAll => {
            show_brands().map(BrandResult::Brands)
        }
    }
}

fn show_brand_by_name(brand: GetBrand) -> Result<Option<DBBrand>, Error> {
    println!("Showing brand: {:?}", brand);
    use crate::schema::brands::dsl::*;

    let connection = &mut establish_connection();
    
    let brand_result = brands
        .filter(name.eq(brand.name))
        .select(DBBrand::as_select())
        .first(connection)
        .optional();

    match brand_result {
        Ok(brand) => Ok(brand),
        Err(err) => Err(err),
    }
}

fn create_brand(brand: CreateBrand) -> Result<String, Error> {
    println!("Creating brand: {:?}", brand);
    use crate::schema::brands::dsl::*;

    let connection = &mut establish_connection();

    let new_brand = NewBrand {
        name: &brand.name
    };

    let result = diesel::insert_into(brands)
                    .values(new_brand)
                    .execute(connection)
                    .optional();

    match result {
        Ok(brand) => Ok(format!("Creating brand: {:?}", brand)),
        Err(err) => Err(err),
    }
}

fn update_brand(brand: UpdateBrand) -> Result<Option<DBBrand>, Error> {
    println!("Updating brand: {:?}", brand);
    use crate::schema::brands::dsl::*;

    let connection = &mut establish_connection();

    let result = diesel::update(brands.find(brand.id))
                    .set(name.eq(brand.name))
                    .returning(DBBrand::as_returning())
                    .get_result(connection)
                    .optional();
                
    match result {
        Ok(brand) => Ok(brand),
        Err(err) => Err(err),
    }
}

fn delete_brand(brand: DeleteEntity) -> Result<String, Error> {
    println!("Deleting brand: {:?}", brand);
    use crate::schema::brands::dsl::*;

    let connection = &mut establish_connection();

    let result = diesel::delete(brands.find(brand.id))
        .execute(connection)
        .optional();

    match result {
        Ok(_) => Ok("Brand deleted".to_string()),
        Err(err) => Err(err),
    }
}

fn show_brands() -> Result<Vec<DBBrand>, Error> {
    println!("Displaying all brands");
    use crate::schema::brands::dsl::*;

    let connection = &mut establish_connection();

    let result = brands
        .load::<DBBrand>(connection);

    result
}