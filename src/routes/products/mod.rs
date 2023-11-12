use warp::Filter;

pub fn route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("products")
        .and(warp::get())
        .map(|| "Products endpoint")
}


pub use route as products_route;
