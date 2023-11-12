use warp::Filter;

pub fn route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("sub_departments")
        .and(warp::get())
        .map(|| "SubDepartments endpoint")
}

pub use route as sub_departments_route;