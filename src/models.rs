use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::brands)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Brand {
    pub id: i32,
    pub name: String,
    pub url_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::brands)]
pub struct UpdateBrand {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::brands)]
pub struct NewBrand <'a> {
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::brands)]
pub struct RemoveBrand {
    pub id: i32,
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::departments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Department {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub color: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::departments)]
pub struct NewDepartment <'a> {
    pub name: &'a str,
    pub color: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::departments)]
pub struct UpdateDepartment {
    pub id: i32,
    pub name: String,
    pub color: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::departments)]
pub struct RemoveDepartment {
    pub id: i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::sub_departments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SubDepartment {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub department_id: Option<i32>, 
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub price: Option<f64>,
    pub description: String,
    pub images: Vec<Option<String>>,
    pub brand_id: Option<i32>,
    pub department_id: Option<i32>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::product_sub_departments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductSubDepartment {
    pub product_id: i32,
    pub sub_department_id: i32,
}