mod sub_department;

use warp::Filter;

pub fn route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("departments")
        .and(warp::get())
        .map(|| "Departments endpoint")
}


use route as departments_route;
use sub_department::route as sub_departments_route;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    departments_route().or(sub_departments_route())
}