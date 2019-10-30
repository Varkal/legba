use crate::fairings::entity_manager::EntityManager;
use crate::forms::medium_post::CreateMediumPostForm;
use crate::models::medium_post::MediumPost;
use rocket::Route;
use rocket_contrib::json::Json;

#[get("/")]
pub fn index() -> Json<Vec<MediumPost>> {
    let entity_manager = EntityManager::<MediumPost>::new(String::from("test-legba-2"));
    return Json(entity_manager.get_all().unwrap_or_default());
}

#[derive(Responder)]
pub enum CreateArticleResponder {
    #[response(status = 200)]
    Ok(Json<MediumPost>),
    #[response(status = 500)]
    DBError(String),
}

#[post("/", data = "<body>")]
pub fn create_article(body: Json<CreateMediumPostForm>) -> CreateArticleResponder {
    let entity_manager = EntityManager::<MediumPost>::new(String::from("test-legba-2"));

    let medium_post = MediumPost {
        title: body.title.clone(),
        url: body.url.clone(),
        ..Default::default()
    };

    match entity_manager.put_item(medium_post.clone()) {
        Ok(_result) => return CreateArticleResponder::Ok(Json(medium_post)),
        _ => {
            return CreateArticleResponder::DBError(String::from(
                "An error occured duricaprineng db insertion",
            ))
        }
    };
}

pub fn routes() -> Vec<Route> {
    return routes![index, create_article];
}
