use crate::args::{
    DepartmentSubcommand, 
    DepartmentCommand, 
    GetEntity, 
    CreateWithNameEntity, 
    UpdateDepartment, 
    DeleteEntity
};
use crate::db::establish_connection;
use crate::models::{NewDepartment, Department as DBDepartment};
use diesel::prelude::*;
use diesel::result::Error;
use crate::schema::departments::dsl::*;
use log::info;

pub enum DepartmentResult {
    Department(Option<DBDepartment>),
    Departments(Vec<DBDepartment>),
    Message(String),
}

pub fn handle_department_command(department: DepartmentCommand) -> Result<DepartmentResult, Error> {
    let connection = &mut establish_connection();
    let command = department.command;
    match command {
        DepartmentSubcommand::Show(department) => {
            show_department_by_name(department, connection).map(DepartmentResult::Department)
        }
        DepartmentSubcommand::Create(department) => {
            create_department(department, connection).map(DepartmentResult::Message)
        }
        DepartmentSubcommand::Update(department) => {
            update_department(department, connection).map(DepartmentResult::Department)
        }
        DepartmentSubcommand::Delete(delete_entity) => {
            delete_department(delete_entity, connection).map(DepartmentResult::Message)
        }
        DepartmentSubcommand::ShowAll => {
            show_departments(connection).map(DepartmentResult::Departments)
        }
    }
}

fn show_department_by_name(department: GetEntity, connection: &mut PgConnection) -> Result<Option<DBDepartment>, Error> {
    info!("Showing department: {:?}", department);
    
    let department_result = departments
        .filter(name.eq(department.name))
        .select(DBDepartment::as_select())
        .first(connection)
        .optional();

    department_result
}

fn create_department(department: CreateWithNameEntity, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Creating department: {:?}", department);

    let new_department = NewDepartment {
        name: &department.name
    };

    let result = diesel::insert_into(departments)
                    .values(new_department)
                    .execute(connection)
                    .optional();

    match result {
        Ok(department) => Ok(format!("Creating department: {:?}", department)),
        Err(err) => Err(err),
    }
}

fn update_department(department: UpdateDepartment, connection: &mut PgConnection) -> Result<Option<DBDepartment>, Error> {
    info!("Updating department: {:?}", department);

    let result = diesel::update(departments.find(department.id))
                    .set(name.eq(department.name))
                    .returning(DBDepartment::as_returning())
                    .get_result(connection)
                    .optional();
                
    match result {
        Ok(department) => Ok(department),
        Err(err) => Err(err),
    }
}

fn delete_department(department: DeleteEntity, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Deleting department: {:?}", department);

    let num_deleted = diesel::delete(departments.find(department.id))
        .execute(connection)?;

    match num_deleted {
        0 => Err(Error::NotFound),
        _ => Ok("Department deleted".to_string()),
    }
}

fn show_departments(connection: &mut PgConnection) -> Result<Vec<DBDepartment>, Error> {
    info!("Displaying all departments");

    let result = departments
        .load::<DBDepartment>(connection);

    result
}