use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::Template;

use crate::models::user::AuthenticatedUser;
use crate::models::user::User;

#[post("/login", data = "<user>")]
pub fn logging(jar: &CookieJar<'_>, mut user: Form<User>) -> Flash<Redirect> {
    jar.add_private(Cookie::new("user_id", user.username.clone()));
    user.hash_password();

    Flash::success(Redirect::to("/"), "Welcome!")
}

#[get("/login")]
pub fn logged(_user: AuthenticatedUser) -> Flash<Redirect> {
    Flash::success(Redirect::to("/"), "Welcome!")
}

#[get("/login", rank = 2)]
pub fn login_form() -> Template {
    Template::render("login", "login")
}

#[get("/logout")]
pub fn logout(_user: AuthenticatedUser, jar: &CookieJar<'_>) -> Flash<Redirect> {
    jar.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/login"), "Come back soon!")
}
