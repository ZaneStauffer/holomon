use std::{error::Error, fmt};

// Database Error //
#[derive(Debug, Clone)]
pub struct DB_Error{
    message: String
}

impl Error for DB_Error{}

impl fmt::Display for DB_Error{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.message)
    }
}
impl DB_Error{
    fn new(msg: &str) -> DB_Error{
        DB_Error{message: msg.to_string()}
    }
}
impl From<mongodb::error::Error> for DB_Error{
    fn from(err: mongodb::error::Error) -> Self{
        DB_Error::new(err.description())
    }
}