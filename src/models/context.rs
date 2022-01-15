use rocket::serde::{Serialize, Deserialize};
use crate::models::earthquake::Quake;

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexContext {
    pub user: String,
    pub quakes: Vec<Quake>,
}
