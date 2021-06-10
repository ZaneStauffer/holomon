use std::collections::HashMap;
use mongodb::{Client, options::ClientOptions, Database};
use std::result::Result;

use crate::error::DB_Error;

pub async fn db_connect(config: &HashMap<String, String>) -> Result<Database, DB_Error>{
    let connection_string = format!("mongodb://{username}:{password}@{address}:{port}",
        address=config["db_address"],
        username=config["db_username"],
        password=config["db_password"],
        port=config["db_port"]);
    let mut cl_options = ClientOptions::parse(connection_string.as_str()).await?;
    cl_options.app_name = Some("Holomon-Server".to_string());
    Ok(Client::with_options(cl_options)?.database("holomon"))
}