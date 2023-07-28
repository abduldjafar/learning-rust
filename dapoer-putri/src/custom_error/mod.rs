use std::fmt;


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
