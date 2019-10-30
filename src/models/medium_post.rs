use dynamo_mapper::*;
use serde::{Deserialize, Serialize};
use uuid::*;
use crate::overrides::dynamo_mapper::*;

#[derive(Serialize, Deserialize, Debug, DynamoMapper, Clone)]
pub struct MediumPost {
    pub id: Uuid,
    pub title: String,
    pub url: String,
}

impl Default for MediumPost {
    fn default() -> MediumPost {
        MediumPost {
            id: Uuid::new_v4(),
            title: Default::default(),
            url: Default::default(),
        }
    }
}
