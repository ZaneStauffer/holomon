use rocket::*;
use rocket::response::status::NotFound;
use rocket::response::content;
use rocket::serde::{Serialize, json::Json};
use rocket::http::Status;
use std::result::Result;
use futures::stream::StreamExt;

use mongodb::{
    bson::{doc, Bson},
    options::FindOptions,
};

use crate::db_connection::db_connect;
use crate::models::user::User;
use crate::error::APIKeyError;
use crate::routes::guards;
use crate::CONFIG;





pub fn list_routes() -> Vec<Route>{
    routes![
        //hello_world,
        test,
        create_user
    ]
}

// #[get("/hello")]
// pub async fn hello_world() -> Json<User>{
//     let db = db_connect(&CONFIG).await.unwrap();
    
// }
#[post("/user/create", format="application/json", /* data="<user>" */)]
pub async fn create_user(key: guards::ApiKey<'_>/*, validator: Validator*/){
    println!("success!");
}

#[get("/test")]
pub async fn test() -> (){
    let db = db_connect(&CONFIG).await.unwrap();
    let mut cursor = db.collection("users").find(None, None).await.unwrap();
    while let Some(result) = cursor.next().await{
        match result{
            Ok(document) => {
                println!("{}", document);
            }
            Err(e) => {

            },
        }
    }
}