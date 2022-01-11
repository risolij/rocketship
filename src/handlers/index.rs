use rocket_dyn_templates::Template;
use rocket::response::{Flash, Redirect};
use rocket::http::CookieJar;
use std::collections::HashMap;

use crate::models::user::AuthenticatedUser;


#[get("/")]
pub fn index_logged(_user: AuthenticatedUser, jar: &CookieJar<'_>) -> Template {
    let mut context: HashMap<String, String> = HashMap::new();
    context.insert(
        "user".to_string(), 
        jar.get_private("user_id").map(|cookie| cookie.value().to_string()).unwrap()
    );

    Template::render("index", &context)
}


#[get("/", rank = 2)]
pub fn index() -> Flash<Redirect> {
    Flash::error(Redirect::to("/login"), "Please login first")
}
