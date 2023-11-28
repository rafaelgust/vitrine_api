use crate::args::{
    SubDepartmentSubcommand, 
    SubDepartmentCommand, 
    GetEntity, 
    CreateSubDepartment, 
    UpdateSubDepartment, 
    DeleteEntity
};
use crate::db::establish_connection;
use crate::models::{NewSubDepartment, SubDepartment as DBSubDepartment};
use diesel::prelude::*;
use diesel::result::Error;
use crate::schema::sub_departments::dsl::*;
use log::info;

pub enum SubDepartmentResult {
    SubDepartment(Option<DBSubDepartment>),
    SubDepartments(Vec<DBSubDepartment>),
    Message(String),
}

pub fn handle_sub_department_command(sub_department: SubDepartmentCommand) -> Result<SubDepartmentResult, Error> {
    let connection = &mut establish_connection();
    let command = sub_department.command;
    match command {
        SubDepartmentSubcommand::Show(sub_department) => {
            show_sub_department_by_name(sub_department, connection).map(SubDepartmentResult::SubDepartment)
        }
        SubDepartmentSubcommand::Create(sub_department) => {
            create_sub_department(sub_department, connection).map(SubDepartmentResult::Message)
        }
        SubDepartmentSubcommand::Update(sub_department) => {
            update_sub_department(sub_department, connection).map(SubDepartmentResult::SubDepartment)
        }
        SubDepartmentSubcommand::Delete(delete_entity) => {
            delete_sub_department(delete_entity, connection).map(SubDepartmentResult::Message)
        }
        SubDepartmentSubcommand::ShowAll => {
            show_sub_departments(connection).map(SubDepartmentResult::SubDepartments)
        }
    }
}

fn show_sub_department_by_name(sub_department: GetEntity, connection: &mut PgConnection) -> Result<Option<DBSubDepartment>, Error> {
    info!("Showing sub_department: {:?}", sub_department);
    
    let sub_department_result = sub_departments
        .filter(name.eq(sub_department.name))
        .select(DBSubDepartment::as_select())
        .first(connection)
        .optional();

    sub_department_result
}

fn create_sub_department(sub_department: CreateSubDepartment, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Creating sub_department: {:?}", sub_department);

    let new_sub_department = NewSubDepartment {
        name: &sub_department.name,
        department_id: &sub_department.department_id,
    };

    let result = diesel::insert_into(sub_departments)
                    .values(new_sub_department)
                    .execute(connection)
                    .optional();

    match result {
        Ok(sub_department) => Ok(format!("Creating sub_department: {:?}", sub_department)),
        Err(err) => Err(err),
    }
}

fn update_sub_department(sub_department: UpdateSubDepartment, connection: &mut PgConnection) -> Result<Option<DBSubDepartment>, Error> {
    info!("Updating sub_department: {:?}", sub_department);

    let result = diesel::update(sub_departments.find(sub_department.id))
                    .set((name.eq(sub_department.name), department_id.eq(sub_department.department_id)))
                    .returning(DBSubDepartment::as_returning())
                    .get_result(connection)
                    .optional();
                
    match result {
        Ok(sub_department) => Ok(sub_department),
        Err(err) => Err(err),
    }
}

fn delete_sub_department(sub_department: DeleteEntity, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Deleting sub_department: {:?}", sub_department);

    let num_deleted = diesel::delete(sub_departments.find(sub_department.id))
        .execute(connection)?;

    match num_deleted {
        0 => Err(Error::NotFound),
        _ => Ok("SubDepartment deleted".to_string()),
    }
}

fn show_sub_departments(connection: &mut PgConnection) -> Result<Vec<DBSubDepartment>, Error> {
    info!("Displaying all sub_departments");

    let result = sub_departments
        .load::<DBSubDepartment>(connection);

    result
}