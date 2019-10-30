use rocket::Route;

pub mod articles;

pub fn routes() -> Vec<Route> {
    let mut routes_vec = Vec::<Route>::new();

    routes_vec.extend(articles::routes());

    return routes_vec;
}
