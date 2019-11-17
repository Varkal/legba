use reqwest;
use rocket::Route;
use scraper;
use std::env;

#[post("/import")]
pub fn import() -> String {
    let client = reqwest::Client::new();
    let result = client
        .get("https://medium.com/me/list/queue?limit=2")
        .header("cookie", env::var("LEGBA_COOKIE").unwrap_or_default())
        .send();

    let html_text = result.unwrap().text().unwrap();

    let document = scraper::Html::parse_document(html_text.as_str());
    let article_selector = scraper::Selector::parse(".streamItem").unwrap();
    let title_selector = scraper::Selector::parse("h3").unwrap();

    return document
        .select(&article_selector)
        .flat_map(|article_element| {
            article_element
                .select(&title_selector)
                .map(|title_element| title_element.text().collect::<String>())
        })
        .collect::<Vec<String>>().join("\n");
}

pub fn routes() -> Vec<Route> {
    return routes![import];
}
