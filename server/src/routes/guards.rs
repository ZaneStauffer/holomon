use rocket::request::{Request, FromRequest, Outcome};
use rocket::data::{self, Data, FromData};
use rocket::http::Status;
use crate::error::APIKeyError;
use crate::CONFIG;

pub struct ApiKey<'r>(&'r str);
// pub struct JSONValidator<'r>();

//Request guard should validate API key and schemas
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r>{
    type Error = APIKeyError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error>{
        fn is_valid_key(key: &str) -> bool{
            key == &CONFIG["api_key"]
        }

        match request.headers().get_one("x-api-key"){
            None => Outcome::Failure((Status::BadRequest, APIKeyError::Missing)),
            Some(key) if is_valid_key(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, APIKeyError::Invalid))
        }
    }
}