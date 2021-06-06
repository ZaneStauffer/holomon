use warp::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use toml::{de::Error};

#[macro_use]
extern crate clap;
use clap::{Arg, App};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    name: String,
    quantity: i32
}

#[tokio::main]
async fn main(){
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

    let holomon_config: HashMap<String, String>;
    match read_config(config){
        Ok(c) => {
            holomon_config = c;
        },
        Err(_) => {
            panic!("There was a problem loading the config.");
        }
    };
    println!("{:#?}", holomon_config);

    let mut res = HashMap::new();
    res.insert("test", Item{name: "testaaaa".to_string(), quantity: 20});

    

    let hello = warp::path!("hello")
        .map(move || {
            warp::reply::json(&res)
        });
    
    warp::serve(hello)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

fn read_config(path: &str) -> Result<HashMap<String, String>, Error>{
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