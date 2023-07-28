use std::env;

use crate::custom_error::{CustomError, ERROR_DATABASE_CONNECTION, ERROR_GET_DATA_FROM_DB};

use super::{data_sources::Datasource, User};
use async_trait::async_trait;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    error::Error,
    options::{ClientOptions, FindOneOptions},
    Client,
};

enum Table {
    User,
}

enum Database {
    DapoerPoetri,
}

pub struct MongoSource {}

fn choose_table(table: Table) -> String {
    match table {
        Table::User => "users".to_string(),
    }
}

fn choose_database(database: Database) -> String {
    match database {
        Database::DapoerPoetri => "dapoer-poetri".to_string(),
    }
}

async fn connection() -> Result<Client, Error> {
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let client_options = ClientOptions::parse(client_uri).await?;

    // Create a new client and connect to the server
    let client = Client::with_options(client_options);
    client
}

#[async_trait]
impl Datasource for MongoSource {
    async fn get_user(&self, id: String) -> Result<Option<super::User>, CustomError> {
        let conn_result = connection().await;

        let conn = match conn_result {
            Ok(conn) => conn,
            Err(_) => return Err(CustomError{
                message: ERROR_DATABASE_CONNECTION.to_string()
            }),
        };

        let collection: mongodb::Collection<Document> = conn
            .database(&choose_database(Database::DapoerPoetri))
            .collection(&choose_table(Table::User));

        let object_id_tmp = ObjectId::parse_str(&id);

        match object_id_tmp {
            Ok(id) => {
                let query = doc! {
                    "_id": id
                };

                let options = FindOneOptions::builder().build();

                let bson_doc_result = collection.find_one(query, options).await;

                let bson_doc = match bson_doc_result {
                    Ok(bson_result) => bson_result,
                    Err(_) => return Err(CustomError{
                        message:ERROR_GET_DATA_FROM_DB.to_string()
                    }),
                };

                let bson_bytes = bson::to_vec(&bson_doc).unwrap();

                let user: User = bson::from_slice(&bson_bytes).unwrap();

                Ok(Some(user))
            }
            Err(_) => Ok(None),
        }

    }

    async fn get_users(&self) -> Vec<super::User> {
        todo!()
    }

    async fn add_user(&self, user: User) -> Result<(), CustomError> {
        let conn_result = connection().await;

        let conn = match conn_result {
            Ok(conn) => conn,
            Err(_) => return Err(CustomError{
                message: ERROR_DATABASE_CONNECTION.to_string()
            }),
        };

        let collection: mongodb::Collection<Document> = conn
            .database(&choose_database(Database::DapoerPoetri))
            .collection(&choose_table(Table::User));

        let bson_user = bson::to_document(&user).expect("Failed to convert to BSON Document");
        let _ = collection.insert_one(bson_user, None).await;
        Ok(())
    }

    async fn delete_user(&self) {
        todo!()
    }
}
