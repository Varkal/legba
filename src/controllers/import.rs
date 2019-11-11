use rocket::Route;
// use reqwest::

#[post("/import")]
pub fn import() -> String {
    return String::from("toto");
}

pub fn routes() -> Vec<Route> {
    return routes![import];
}
