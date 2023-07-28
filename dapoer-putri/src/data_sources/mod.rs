pub mod mongo_source;
pub mod postgres_source;
pub mod data_sources;
pub mod initialization;

use serde_derive::Deserialize;
use mongodb::bson::oid::ObjectId;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
     pub id: Option<ObjectId>,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "gender")]
    pub gender: String
}
