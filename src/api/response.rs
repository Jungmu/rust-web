use serde::{Deserialize, Serialize};

use crate::db::models;

#[derive(Deserialize, Serialize)]
pub struct ResLoginAdmin {
    pub id: String,
}

pub fn parse(model: models::Admin) -> ResLoginAdmin {
    let id = model.id;
    ResLoginAdmin { id }
}
