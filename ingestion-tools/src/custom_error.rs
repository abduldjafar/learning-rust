use std::fmt;
use mongodb::error::Error as MongoError;
use mongodb::bson::oid::Error as BsonOidError;
use tokio::task::JoinError;
use std::error::Error;


pub const ERROR_DATABASE_CONNECTION: &str = "getting error connection to database";
pub const ERROR_GET_DATA_FROM_DB: &str = "getting error when collect data from database";


#[derive(Debug)]
pub struct CustomError{
    pub message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error with  message : {}",self.message)
    }
}

impl From<MongoError> for CustomError {
    fn from(error: MongoError) -> Self {
        CustomError {
            message: error.to_string(),
        }
    }
}

impl From<BsonOidError> for CustomError {
    fn from(error: BsonOidError) -> Self {
        CustomError {
            message: error.to_string(),
        }
    }
}

impl From<google_cloud_storage::http::Error> for CustomError {
    fn from(error: google_cloud_storage::http::Error) -> Self {
        CustomError {
            message: error.to_string(),
        }
    }
}



impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError {
            message: error.to_string(),
        }
    }
}

impl From<google_cloud_storage::client::google_cloud_auth::error::Error> for CustomError {
    fn from(error: google_cloud_storage::client::google_cloud_auth::error::Error) -> Self {
        CustomError {
            message: error.to_string(),
        }
    }
}

impl From<JoinError> for CustomError {
    fn from(error: JoinError) -> Self {
        CustomError {
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error>for CustomError {
    fn from(error: serde_json::Error) -> Self {
        CustomError {
            message: error.to_string(),
        }
    }
}

impl Error for CustomError {}
