use warp::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

type Items = HashMap<String, i32>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    name: String,
    quantity: i32
}

#[tokio::main]
async fn main(){
    let mut res = HashMap::new();
    res.insert("test", Item{name: "testaaaa".to_string(), quantity: 20});

    let hello = warp::path!("hello" / String)
        .map(move |name| {
            warp::reply::json(&res)
        });
    
    warp::serve(hello)
        .run(([0, 0, 0, 0], 3030))
        .await;
}