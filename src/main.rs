use warp::Filter;

mod routes;

#[tokio::main]
async fn main() {
    let routes = routes::products_route().or(routes::routes());

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}