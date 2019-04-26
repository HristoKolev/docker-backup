use std::any::Any;

#[derive(From, Debug)]
pub enum GeneralError {
    IoError(std::io::Error),
    Dynamic(Box<dyn Any + Send + 'static>),
    ParseInt(std::num::ParseIntError),
}
