use warp::Filter;

mod routes;

#[tokio::main]
async fn main() {
    
    //let routes = routes::products_route().or(routes::routes()); // This is the original code

    let products = warp::path!("api" / "products")
    .and(warp::get())
    .map(|| "Products endpoint");

    

    let department = warp::path!("api" / "department" / i32 )
    .map(|department_id: i32| {
        format!("Department ID: {}", department_id)
    });

    let sub_departments = warp::path!("api" / "department" / i32 / "subdepartment" / i32)
    .map(|department_id: i32, subdepartment_id: i32| {
        format!("Department ID: {},
         Subdepartment ID: {}", department_id, subdepartment_id)
    });


    let routes = products.or(department).or(sub_departments);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

