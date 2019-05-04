use std::io::Write;
use crate::errors::CustomErrorKind::{ErrorMessage, IoError, SmtpError, Failure, JsonError, HandlebarsError};
use std::convert::From;

#[derive(Debug)]
pub enum CustomErrorKind {

    ErrorMessage(String),

    IoError(std::io::Error),
    SmtpError(lettre::smtp::error::Error),
    Failure(failure::Error),
    JsonError(serde_json::Error),
    HandlebarsError(handlebars::TemplateRenderError),
}

#[derive(Debug)]
pub struct CustomError {
    kind: CustomErrorKind,
}

pub type Result<T> = std::result::Result<T, CustomError>;

impl CustomError {
    pub fn from_message(message: &str) -> CustomError {
        CustomError {
            kind: ErrorMessage(message.to_string())
        }
    }
}

pub fn handle_error(error: &CustomError) -> Result<()> {

    let stderr = &mut ::std::io::stderr();

    writeln!(stderr, "error: {:?}", error)?;

    Ok(())
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        CustomError {
            kind: IoError(err)
        }
    }
}

impl From<lettre::smtp::error::Error> for CustomError {
    fn from(err: lettre::smtp::error::Error) -> Self {
        CustomError {
            kind: SmtpError(err)
        }
    }
}

impl From<failure::Error> for CustomError {
    fn from(err: failure::Error) -> Self {
        CustomError {
            kind: Failure(err)
        }
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(err: serde_json::Error) -> Self {
        CustomError {
            kind: JsonError(err)
        }
    }
}

impl From<handlebars::TemplateRenderError> for CustomError {
    fn from(err: handlebars::TemplateRenderError) -> Self {
        CustomError {
            kind: HandlebarsError(err)
        }
    }
}

pub trait ResultExtensions<R> {
    fn replace_error<ErrFunc>(self, err_func: ErrFunc) -> Result<R>
        where ErrFunc: FnOnce() -> CustomError;
}

impl<R, E> ResultExtensions<R> for std::result::Result<R, E> {
    fn replace_error<ErrFunc>(self, err_func: ErrFunc) -> Result<R>
        where ErrFunc: FnOnce() -> CustomError {
        match self {
            Ok(res) => Ok(res),
            Err(_) => Err(err_func())
        }
    }
}