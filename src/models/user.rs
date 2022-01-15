use crate::rocket::outcome::IntoOutcome;
use argon2::Config;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

#[derive(FromForm)]
pub struct User {
    pub username: String,

    #[field(validate = len(4..))]
    #[field(validate = omits("password"))]
    #[field(validate = contains(&['@', '!', '#', '$', '&']))]
    password: String,
}

impl User {
    pub fn hash_password(&mut self) {
        let password = self.password.as_bytes();
        let salt = b"randomsalt";
        let config = Config::default();
        let hash = argon2::hash_encoded(password, salt, &config).unwrap();

        match self.validate_hash(&hash) {
            Ok(v) => match v {
                true => {
                    self.password = hash;
                    println!("{}", self.password)
                }
                false => {
                    println!("Hashes don't match for some reason")
                }
            },
            Err(e) => println!("Error {}", e),
        }
    }

    pub fn validate_hash(&mut self, hash: &String) -> Result<bool, argon2::Error> {
        argon2::verify_encoded(&hash, self.password.as_bytes())
    }
}

pub struct AuthenticatedUser(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<AuthenticatedUser, ()> {
        request
            .cookies()
            .get_private("user_id")
            .map(|id| AuthenticatedUser(id.value().to_string()))
            .or_forward(())
    }
}
