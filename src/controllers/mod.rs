use rocket::Route;

pub mod articles;
pub mod import;

pub fn routes() -> Vec<Route> {
    let mut routes_vec = Vec::<Route>::new();

    routes_vec.extend(articles::routes());
    routes_vec.extend(import::routes());

    return routes_vec;
}
