use rocket_dyn_templates::Template;
use rocket::response::{Flash, Redirect};
use rocket::http::CookieJar;
use rocket::serde::{Serialize, Deserialize};

use crate::models::user::AuthenticatedUser;
use crate::models::query_builder::QueryBuilder;
use crate::models::earthquake::Earthquake;
use crate::models::earthquake::Quake;


#[derive(Debug, Serialize, Deserialize)]
pub struct IndexContext {
    user: String,
    quakes: Vec<Quake>,
}


#[get("/")]
pub async fn index_logged(_user: AuthenticatedUser, jar: &CookieJar<'_>) -> Template {
    let mut query = QueryBuilder::new("2014-01-01", "2014-01-30", Some(5));
    let earthquake: Earthquake = query.build_quake().await.unwrap();

    let context = IndexContext {
        user: jar.get_private("user_id").map(|cookie| cookie.value().to_string()).unwrap(),
        quakes: earthquake.features.unwrap(),
    };

    Template::render("index", &context)
}


#[get("/", rank = 2)]
pub fn index() -> Flash<Redirect> {
    Flash::error(Redirect::to("/login"), "Please login first")
}
