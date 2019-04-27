use std::any::Any;
use std::error;

#[derive(Debug, From)]
pub enum GeneralError {
    IoError(std::io::Error),
    Dynamic(Box<dyn Any + Send + 'static>),
    Detailed(DetailedError),
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

impl error::Error for DetailedError {
    fn description(&self) -> &str {
        &self.message
    }
}
