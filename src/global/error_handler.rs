use super::prelude::*;
use super::sentry_client;
use super::logger;
use super::email_report;

/// The default error handler.
pub fn handle_error(error: &CustomError) -> Result {

    let log_result = logger().log(&format!("An error occurred: {:#?}", error));
    let sentry_result = sentry_client().send_error(error);

    log_result?;
    sentry_result?;

    Ok(())
}

pub fn handle_fatal_error(error: &CustomError) -> Result {

    let standard_error_handler_result = handle_error(error);
    let email_result = email_report::send_error_report(&error);

    standard_error_handler_result?;
    email_result?;

    Ok(())
}

pub trait ResultExtensionsCrashOnError<R> {

    fn crash_on_error(self) -> R;
    fn crash_on_early_error(self) -> R;
}

impl<R> ResultExtensionsCrashOnError<R> for Result<R> {

    fn crash_on_error(self) -> R {

        self.unwrap_or_else(|err| {
            handle_fatal_error(&err)
                .expect(&format!("An error occurred while handling an error. {:#?}", &err));

            ::std::process::exit(1)
        })
    }

    // This is called when an error occurs while initializing the global state.
    // Using the normal error handler to handle the error causes
    // (initialization -> error -> initialization -> error) cycle because it depends on the state to be initialized,
    // because the global state is being "lazily" initialized by `lazy_static`.
    // The easiest way for this to occur is to have invalid json in the config file.
    // Not being able to access the state leaves me with printing to stderr as the only option for handling the error.
    fn crash_on_early_error(self) -> R {

        self.unwrap_or_else(|err| {

            eprintln!("{:#?}", err);

            ::std::process::exit(1)
        })
    }
}
