

use crate::custom_error::CustomError;

use super::User;
use super::mongo_source::MongoSource;
use async_trait::async_trait;

#[async_trait]
pub trait Datasource {
    async fn get_user(&self,id:String) -> Result<Option<User>,CustomError>;
    async fn get_users(&self) -> Vec<User>;
    async fn add_user(&self,user:User) -> Result<(),CustomError>;
    async fn delete_user(&self);
}


pub enum DataSourceTypes {
    MongoSource(MongoSource)
}




#[async_trait]
impl Datasource for DataSourceTypes {
    async fn get_user(&self,id: String) -> Result<Option<User>,CustomError> {
        match *self {
            DataSourceTypes::MongoSource(ref mongo_source) => mongo_source.get_user(id).await
        }
    }

    async fn get_users(&self) -> Vec<User> {
        todo!()
    }

    async fn add_user(&self,user: User) -> Result<(),CustomError> {
        match *self {
            DataSourceTypes::MongoSource(ref mongo_source) => mongo_source.add_user(user).await
        }
    }

    async fn delete_user(&self) {
        todo!()
    }
}