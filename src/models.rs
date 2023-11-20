use diesel::Queryable;

#[derive(Queryable)]
pub struct Product {
  pub id: i32,
  pub name: String,
  pub price: f64,
}