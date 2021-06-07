use serde::{Serialize, Deserialize};
use wither::{prelude::*, Result};
use wither::bson::{doc, oid::ObjectId};
use wither::mongodb::Client;

#[derive(Debug, Model, Serialize, Deserialize)]
#[model(index(
    keys=r#"doc!{"name":1}"#,
    options=r#"doc!{"unique": true}"#
))]
pub struct User{
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub discord_id: String
}