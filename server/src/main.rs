#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use toml::{de::Error};

#[macro_use]
extern crate clap;
use clap::{Arg, App};
use std::path::Path;

use wither::{prelude::*, Result};
use wither::bson::{doc, oid::ObjectId};
use wither::mongodb::Client;

mod routes;
mod models;
mod db_connection;
use crate::routes::app_routes;
use crate::models::app_models;



#[tokio::main]
async fn main() -> Result<()>{
    // Parse CLI args
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
    let config: &str;
    match matches.value_of("config"){
        Some(c) => {
            config = c;
        }
        None => {
            panic!("Please supply a config toml.");
        }
    };
    // Grab config
    let holomon_config: HashMap<String, String>;
    match read_config(config){
        Ok(c) => {
            holomon_config = c;
        },
        Err(_) => {
            panic!("There was a problem loading the config.");
        }
    };

    let db = db_connection::db_connect(
        holomon_config["db_address"].as_str(),
        holomon_config["db_user"].as_str(),
        holomon_config["db_password"].as_str(),
        holomon_config["db_port"].as_str()
    ).await?;

    app_models::User::sync(&db).await?;

    let mut me = app_models::User{
        id: None,
        name: String::from("Zane"),
        discord_id: String::from("AAFEFFFF")
    };

    me.save(&db, None).await?;

    // Launch rocket (get it?)
    rocket::ignite().mount("/", routes![
        app_routes::hello_world
    ]).launch();

    Ok(())
}

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