use super::{data_sources::Datasource, User};
use crate::custom_error::CustomError;
use async_trait::async_trait;
use futures::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    options::{ClientOptions, FindOneOptions, FindOptions},
    Client,
};
use std::env;

enum Table {
    User,
}

enum Database {
    DapoerPoetri,
}

pub struct MongoSource {}

impl MongoSource {
    async fn get_collection(
        &self,
        table: Table,
    ) -> Result<mongodb::Collection<Document>, CustomError> {
        let conn = self.connection().await?;
        let database_name = match Database::DapoerPoetri {
            Database::DapoerPoetri => "dapoer-poetri",
        };
        let collection_name = match table {
            Table::User => "users",
        };
        Ok(conn.database(database_name).collection(collection_name))
    }

    async fn connection(&self) -> Result<Client, CustomError> {
        let client_uri =
            env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
        let client_options = ClientOptions::parse(client_uri).await?;
        let client = Client::with_options(client_options)?;
    
        Ok(client)
    }
}


#[async_trait]
impl Datasource for MongoSource {
    async fn get_user(&self, id: String) -> Result<Option<User>, CustomError> {
        let collection = self.get_collection(Table::User).await?;

        let object_id = ObjectId::parse_str(&id)?;

        let query = doc! {
            "_id": object_id
        };

        let options = FindOneOptions::builder().build();
        if let Some(doc) = collection.find_one(query, options).await? {
            Ok(Some(bson::from_document(doc).unwrap()))
        } else {
            Ok(None)
        }
    }

    async fn get_users(&self) -> Result<Vec<User>,CustomError>{
        let mut vector: Vec<User> = Vec::new();
        let collection = self.get_collection(Table::User).await?;
        let mut options = FindOptions::builder().build();
        options.limit = Some(100);
        options.skip = Some(0);
        
        let mut cursor = collection.find(None, options).await?;

        while let Some(doc) = cursor.try_next().await?{
            let mut data:User = bson::from_document(doc).unwrap();
            data.password = "****".to_string();
            vector.push(data);
        }
        Ok(vector)

    }

    async fn add_user(&self, user: User) -> Result<(), CustomError> {
        let collection = self.get_collection(Table::User).await?;
        let bson_user = bson::to_document(&user).expect("Failed to convert to BSON Document");
        collection.insert_one(bson_user, None).await?;
        Ok(())
    }

    async fn delete_user(&self) {
        todo!()
    }
}
