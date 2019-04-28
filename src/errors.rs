use std::any::Any;
use crate::errors::GeneralError::{IoError, Dynamic, Detailed};

#[derive(Debug, From)]
pub enum GeneralError {
    IoError(std::io::Error),
    Dynamic(Box<dyn Any + Send + 'static>),
    Detailed(DetailedError),
}

pub fn handle_error(error: GeneralError) {
    match error {
        IoError(io) => print!("{:#?}", io),
        Dynamic(dy) => print!("{:#?}", dy),
        Detailed(detail) => print!("{:#?}", detail),
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

