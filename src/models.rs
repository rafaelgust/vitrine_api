use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::brands)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Brand {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::brands)]
pub struct NewBrand <'a> {
    name: &'a str,
}

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::departments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Department {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::sub_departments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SubDepartment {
    pub id: i32,
    pub name: String,
    pub department_id: Option<i32>, 
}

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: Option<f64>,
    pub description: String,
    pub images: Vec<Option<String>>,
    pub brand_id: Option<i32>,
    pub department_id: Option<i32>,
}

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::product_sub_departments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductSubDepartment {
    pub product_id: i32,
    pub sub_department_id: i32,
}