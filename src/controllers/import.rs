use reqwest;
use scraper;
use rocket::Route;
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

    let mut articles_title = Vec::<String>::new();

    for article_element in document.select(&article_selector) {
        for title_element in article_element.select(&title_selector) {
            articles_title.push(title_element.text().collect())
        }
    }

    return articles_title.join("\n");
}

pub fn routes() -> Vec<Route> {
    return routes![import];
}
