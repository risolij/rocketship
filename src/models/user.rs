use rocket::request::{FromRequest, Outcome};
use crate::rocket::outcome::IntoOutcome;
use rocket::Request;
use argon2::Config;


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
            true => {
                self.password = hash;
                println!("{}", self.password);
            },
            false => panic!("Something went wrong"),
        }
    }


    pub fn validate_hash(&mut self, hash: &String) -> bool {
        argon2::verify_encoded(&hash, self.password.as_bytes()).unwrap()
    }
}


pub struct AuthenticatedUser(String);


#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<AuthenticatedUser, ()> {
        request.cookies()
            .get_private("user_id")
            .map(|id| AuthenticatedUser(id.value().to_string()))
            .or_forward(())
    }
}
