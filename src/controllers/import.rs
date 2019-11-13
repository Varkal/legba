use reqwest;
use rocket::Route;
use std::env;

#[post("/import")]
pub fn import() -> String {
    let client = reqwest::Client::new();
    let result = client
        .get("https://medium.com/me/list/queue?limit=2")
        .header("cookie", env::var("LEGBA_COOKIE").unwrap_or_default())
        .send();
    return result.unwrap().text().unwrap();
}

pub fn routes() -> Vec<Route> {
    return routes![import];
}
