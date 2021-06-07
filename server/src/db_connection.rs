use wither::{prelude::*, Result};
use wither::bson::{doc, oid::ObjectId};
use wither::mongodb::Client;

pub async fn db_connect(
    addr: &str,
    usr: &str,
    pass: &str,
    port: &str) -> Result<wither::mongodb::Database>
{
    println!("{}, {}, {}, {}", addr, usr, pass, port);
    Ok(Client::with_uri_str(format!("mongodb://{username}:{password}@{address}:{port}",
        address=addr, username=usr, password=pass, port=port).as_str())
        .await?.database("holomon"))
}