use std::fmt;
use mongodb::error::Error as MongoError;
use mongodb::bson::oid::Error as BsonOidError;


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