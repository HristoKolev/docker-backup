use std::any::Any;
use crate::errors::GeneralError::{IoError, Dynamic, Detailed, ParseInt, SmtpError, Failure, JsonError};
use std::num::ParseIntError;

#[derive(Debug, From)]
pub enum GeneralError {
    IoError(std::io::Error),
    Dynamic(Box<dyn Any + Send + 'static>),
    Detailed(DetailedError),
    ParseInt(ParseIntError),
    SmtpError(lettre::smtp::error::Error),
    Failure(failure::Error),
    JsonError(serde_json::Error),
}

pub fn handle_error(error: GeneralError) {
    match error {
        IoError(err) => print!("{:#?}", err),
        Dynamic(err) => print!("{:#?}", err),
        Detailed(err) => print!("{:#?}", err),
        ParseInt(err) => print!("{:#?}", err),
        SmtpError(err) => print!("{:#?}", err),
        Failure(err) => print!("{:#?}", err),
        JsonError(err) => print!("{:#?}", err),
    }
}

#[derive(Debug, Display, Clone)]
pub struct DetailedError {
    message: String,
}

impl DetailedError {
    pub fn new(message: String) -> DetailedError {
        DetailedError {
            message: format!("An error occurred: {}", message)
        }
    }
}

impl std::error::Error for DetailedError {
    fn description(&self) -> &str {
        &self.message
    }
}

