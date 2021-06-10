use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;


#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub discord_id: String
}