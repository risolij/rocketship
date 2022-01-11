use rocket::response::{Flash, Redirect};


#[catch(404)]
pub fn not_found() -> Flash<Redirect> {
    Flash::error(Redirect::to("/"), "404 not found - rerouting")
}
