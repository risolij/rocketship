#[macro_use] extern crate rocket;
// #[macro_use] extern crate diesel;
extern crate argon2;


mod handlers;
mod models;
mod lib;


use rocket_dyn_templates::Template;
use lib::shield_wall;


#[launch]
fn rocket() -> _ {
    let shield = shield_wall();

    rocket::build()
        .attach(shield)
        .attach(Template::fairing())
        .register("/", catchers![handlers::catchers::not_found])
        .mount("/", routes![
            handlers::index::index,
            handlers::index::index_logged,
        ])
        .mount("/", routes![
            handlers::login::login_form,
            handlers::login::logging,
            handlers::login::logged,
            handlers::login::logout,
        ])
}
