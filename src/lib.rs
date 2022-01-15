use rocket::shield::{Referrer, Shield, XssFilter};

pub fn shield_wall() -> Shield {
    Shield::default()
        .enable(XssFilter::EnableBlock)
        .enable(Referrer::NoReferrer)
}
