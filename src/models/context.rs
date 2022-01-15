use crate::models::earthquake::Quake;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexContext {
    pub user: String,
    pub quakes: Vec<Quake>,
}
