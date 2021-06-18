use rocket::request::{Request, FromRequest, Outcome};
use rocket::data::{self, Data, FromData};
use rocket::http::Status;
use rocket::serde::{Serialize, json::Json};
use crate::error::{APIKeyError, ValidationError};
use crate::CONFIG;

pub struct ApiKey<'r>(&'r str);
pub struct JSONValidator<'r>(&'r Json<&'r str>);

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

// #[rocket::async_trait]
// impl <'r> FromData<'r> for JSONValidator<'r>{
//     type Error = ValidationError;
//     async fn from_data(request: &'r Request<'_>, data: Data) -> data::Outcome<Self, ValidationError>{
//         let limit = request.limits().get("json")
//     }
// }