use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CreateMediumPostForm {
    pub title: String,
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateMediumPostForm {
    pub id: Uuid,
    pub title: String,
    pub url: String,
}
