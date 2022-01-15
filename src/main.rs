#[macro_use] extern crate rocket;
extern crate argon2;
// #[macro_use] extern crate diesel;


mod handlers;
mod models;
mod lib;


use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use lib::shield_wall;


#[launch]
fn rocket() -> _ {
    let shield = shield_wall();

    rocket::build()
        .attach(shield)
        .attach(Template::fairing())
        .register("/", catchers![handlers::catchers::not_found])
        .mount("/static", FileServer::from("static/"))
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
