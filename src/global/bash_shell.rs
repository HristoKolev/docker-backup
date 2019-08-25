use std::process::{Command, Stdio};
use std::thread::JoinHandle;
use std::thread;
use std::io::{BufReader, Write, BufRead};

use super::prelude::*;

fn exec_internal(command: &str, log_output: bool) -> Result<CommandResult> {

    let mut process = Command::new("/usr/bin/env")
        .arg("bash")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;

    let stdout = process.stdout.take()
        .or_error("stdout was not redirected.")?;

    let stderr = process.stderr.take()
        .or_error("stderr was not redirected.")?;

    let stdin = process.stdin.as_mut()
        .or_error("stdin was not redirected.")?;

    let stdout_thread : JoinHandle<Result<String>> = thread::spawn(move || {

        let buff = BufReader::new(stdout);

        let mut result = String::new();

        for line_result in buff.lines() {

            let line = line_result?;
            result.push_str(&format!("{}\n", line));

            if log_output {
                logger().log(&format!("OUT | {}", line))?;
            }
        }

        Ok(result)
    });

    let stderr_thread : JoinHandle<Result<String>> = thread::spawn(move || {

        let buff = BufReader::new(stderr);

        let mut result = String::new();

        for line_result in buff.lines() {

            let line = line_result?;
            result.push_str(&format!("{}\n", line));

            logger().log(&format!("ERR | {}", line))?;
        }

        Ok(result)
    });

    stdin.write_all("set -exu\n".as_bytes())?;
    stdin.write_all(format!("{}\n", command).as_bytes())?;
    stdin.write_all("exit $?;\n".as_bytes())?;

    let out_result = stdout_thread.join()
        .on_error("The stdout thread failed for some reason.")??;

    let err_result = stderr_thread.join()
        .on_error("The stderr thread failed for some reason.")??;

    let exit_status = process.wait()?;

    return Ok(CommandResult {
        status_code: exit_status.code(),
        success: exit_status.success(),
        stdout: out_result,
        stderr: err_result,
        command: command.to_string()
    });
}

pub fn exec(command: &str) -> Result<CommandResult> {

    exec_internal(command, true)
}

pub fn exec_without_log(command: &str) -> Result<CommandResult> {

    exec_internal(command, false)
}


#[derive(Debug)]
pub struct CommandResult {
    pub status_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub command: String,
    pub success: bool,
}

impl CommandResult {

    //noinspection RsSelfConvention
    pub fn as_result(self) -> Result<CommandResult> {
        if self.success {
            Ok(self)
        } else {
            Err(CustomError::from_message(&format!(
                "A command exited with a non 0 exit code or with a signal. '{}'",
                self.command
            )))
        }
    }

    //noinspection RsSelfConvention
    pub fn as_result_ref(&self) -> Result<&CommandResult> {
        if self.success {
            Ok(self)
        } else {
            Err(CustomError::from_message(&format!(
                "A command exited with a non 0 exit code or with a signal. '{}'",
                self.command
            )))
        }
    }
}

#[allow(unused_macros)]
macro_rules! bash_exec {
    ($x:expr) => {
        crate::global::bash_shell::exec(&format!("{}", $x))?.as_result()?
    };
    ($($x:expr),*) => {
        crate::global::bash_shell::exec(&format!($($x,)*))?.as_result()?
    };
}

#[allow(unused_macros)]
macro_rules! bash_exec_no_log {
    ($x:expr) => {
        crate::global::bash_shell::exec_without_log(&format!("{}", $x))?.as_result()?
    };
    ($($x:expr),*) => {
        crate::global::bash_shell::exec_without_log(&format!($($x,)*))?.as_result()?
    };
}