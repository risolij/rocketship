use rocket::shield::{Shield, XssFilter, Referrer};


pub fn shield_wall() -> Shield {
    Shield::default()
        .enable(XssFilter::EnableBlock)
        .enable(Referrer::NoReferrer)
}
