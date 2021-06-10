use rocket::*;
use rocket::response::status::NotFound;
use rocket::response::content;
use rocket::serde::{Serialize, json::Json};
use std::result::Result;
use futures::stream::StreamExt;

use mongodb::{
    bson::{doc, Bson},
    options::FindOptions,
};

use crate::db_connection::db_connect;
use crate::models::user::User;
use crate::CONFIG;

pub fn list_routes() -> Vec<Route>{
    routes![
        //hello_world,
        test
    ]
}

// #[get("/hello")]
// pub async fn hello_world() -> Json<User>{
//     let db = db_connect(&CONFIG).await.unwrap();
    
// }

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