use warp::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use toml::{de::Error};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    name: String,
    quantity: i32
}

#[tokio::main]
async fn main(){
    let holomon_config: HashMap<String, String>;
    match read_config(){
        Ok(c) => {
            holomon_config = c;
        },
        Err(_) => {
            panic!("There was a problem loading the config.");
        }
    };

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

fn read_config() -> Result<HashMap<String, String>, Error>{
    let mut config = config::Config::default();
    config.merge(config::File::with_name("Config")).unwrap();
    Ok(config.try_into::<HashMap<String, String>>().unwrap())
}