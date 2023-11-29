use crate::args::{
    BrandSubcommand, 
    BrandCommand, 
    GetEntity, 
    CreateWithNameEntity, 
    UpdateBrand, 
    DeleteEntity
};
use crate::db::establish_connection;
use crate::models::{NewBrand, Brand as DBBrand};
use diesel::prelude::*;
use diesel::result::Error;
use crate::schema::brands::dsl::*;
use log::info;

pub enum BrandResult {
    Brand(Option<DBBrand>),
    Brands(Vec<DBBrand>),
    Message(String),
}

pub fn handle_brand_command(brand: BrandCommand) -> Result<BrandResult, Error> {
    let connection = &mut establish_connection();
    let command = brand.command;
    match command {
        BrandSubcommand::Show(brand) => {
            show_brand_by_name(brand, connection).map(BrandResult::Brand)
        }
        BrandSubcommand::Create(brand) => {
            create_brand(brand, connection).map(BrandResult::Message)
        }
        BrandSubcommand::Update(brand) => {
            update_brand(brand, connection).map(BrandResult::Brand)
        }
        BrandSubcommand::Delete(delete_entity) => {
            delete_brand(delete_entity, connection).map(BrandResult::Message)
        }
        BrandSubcommand::ShowAll => {
            show_brands(connection).map(BrandResult::Brands)
        }
    }
}

fn show_brand_by_name(brand: GetEntity, connection: &mut PgConnection) -> Result<Option<DBBrand>, Error> {
    info!("Showing brand: {:?}", brand);
    
    let brand_result = brands
        .filter(name.eq(brand.name))
        .select(DBBrand::as_select())
        .first(connection)
        .optional();

    brand_result
}

fn create_brand(brand: CreateWithNameEntity, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Creating brand: {:?}", brand);

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

fn update_brand(brand: UpdateBrand, connection: &mut PgConnection) -> Result<Option<DBBrand>, Error> {
    info!("Updating brand: {:?}", brand);

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

fn delete_brand(brand: DeleteEntity, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Deleting brand: {:?}", brand);

    let num_deleted = diesel::delete(brands.find(brand.id))
        .execute(connection)?;

    match num_deleted {
        0 => Err(Error::NotFound),
        _ => Ok("Brand deleted".to_string()),
    }
}

fn show_brands(connection: &mut PgConnection) -> Result<Vec<DBBrand>, Error> {
    info!("Displaying all brands");

    let result = brands
        .load::<DBBrand>(connection);

    result
}