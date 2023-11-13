use warp::Filter;

mod routes;

#[tokio::main]
async fn main() {
    
    //let routes = routes::products_route().or(routes::routes()); // This is the original code

    let products = warp::path("products")
    .and(warp::get())
    .map(|| "Products endpoint");

    let department = warp::path("departments").and(warp::get()).map(|| "Departments endpoint");

    let sub_departments = warp::path("sub_departments").and(warp::get()).map(|| "SubDepartments endpoint");

    let routes = products.or(department).or(sub_departments);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

