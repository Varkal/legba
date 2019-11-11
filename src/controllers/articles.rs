use crate::forms::medium_post::CreateMediumPostForm;
use crate::models::medium_post::MediumPost;
use crate::services::entity_manager::EntityManager;
use rocket::{Route, State};
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(entity_manager: State<EntityManager<MediumPost>>) -> Json<Vec<MediumPost>> {
    return Json(entity_manager.get_all().unwrap_or_default());
}

#[derive(Responder)]
pub enum CreateArticleResponder<'r> {
    #[response(status = 200)]
    Ok(Json<MediumPost>),
    #[response(status = 500)]
    DBError(&'r str),
}

#[post("/", data = "<body>")]
pub fn create_article(
    body: Json<CreateMediumPostForm>,
    entity_manager: State<EntityManager<MediumPost>>,
) -> CreateArticleResponder {
    let medium_post = MediumPost {
        title: body.title.clone(),
        url: body.url.clone(),
        ..Default::default()
    };

    match entity_manager.put_item(medium_post.clone()) {
        Ok(_result) => return CreateArticleResponder::Ok(Json(medium_post)),
        _ => return CreateArticleResponder::DBError("An error occured during db insertion"),
    };
}

pub fn routes() -> Vec<Route> {
    return routes![index, create_article];
}
