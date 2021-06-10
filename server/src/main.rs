#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use toml::{de::Error};

#[macro_use]
extern crate clap;
use clap::{Arg, App};
use std::path::Path;

#[macro_use]
extern crate lazy_static;

mod routes;
mod models;
mod error;
mod db_connection;
use crate::routes::app_routes;
use crate::models::*;

use mongodb::{Client, options::ClientOptions, Database};

lazy_static!{
    pub static ref CONFIG: HashMap<String, String> = {
        let matches = App::new("holomon-server")
            .author("PrivateNomad")
            .about("Server for the Holomon game.")
            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Supplies Config.toml")
                .takes_value(true))
            .get_matches();
        let _config: &str;
        match matches.value_of("config"){
            Some(c) => {
                _config = c;
            }
            None => {
                panic!("Please supply a config toml.");
            }
        };
        let holomon_config: HashMap<String, String>;
        match read_config(_config){
            Ok(c) => {
                holomon_config = c;
            },
            Err(_) => {
                panic!("There was a problem loading the config.");
            }
        };
        holomon_config
    };
}

#[tokio::main]
#[launch]
async fn rocket() -> _ {
    lazy_static::initialize(&CONFIG);

    let connection_string = format!("mongodb://{username}:{password}@{address}:{port}",
        address=&CONFIG["db_address"],
        username=&CONFIG["db_user"],
        password=&CONFIG["db_password"],
        port=&CONFIG["db_port"]);
    let mut cl_options = ClientOptions::parse(connection_string.as_str()).await.unwrap();
    cl_options.app_name = Some("Holomon-Server".to_string());
    let db = Client::with_options(cl_options).unwrap().database("holomon");
    let mut cursor = db.list_collection_names(None).await.unwrap();
    for collection in cursor{
        println!("{}", collection);
    }

    rocket::build().mount("/", app_routes::list_routes())
}

// #[tokio::main]
// async fn main(){
//     lazy_static::initialize(&CONFIG);
    
//     //let db = db_connection::db_connect(db_options).await?;

//     // models::user::User::sync(&db).await?;

//     // let mut me = models::user::User{
//     //     id: None,
//     //     name: String::from("Zane"),
//     //     discord_id: String::from("AAFEFFFF")
//     // };

//     // me.save(&db, None).await?;

//     // Launch rocket (get it?)
// }

fn read_config(path: &str) -> std::result::Result<HashMap<String, String>, Error>{
    let mut config = config::Config::default();
    match config.merge(config::File::from(Path::new(path))){
        Ok(_) => {
            return Ok(config.try_into::<HashMap<String, String>>().unwrap());
        },
        Err(_) => {
            panic!("Please supply a correct config file.");
        }
    }
    
}