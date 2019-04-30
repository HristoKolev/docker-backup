use std::process::{Command, Stdio};
use std::thread::JoinHandle;
use std::thread;
use std::io::{BufReader, Write, BufRead};

use crate::errors::{GeneralError, DetailedError};

pub fn exec(command: &str) -> Result<CommandResult, GeneralError> {

    let mut process = Command::new("/usr/bin/env")
        .arg("bash")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;

    let stdout = process.stdout.take()
        .ok_or_else(|| DetailedError::new("stdout was not redirected.".to_string()))?;

    let stderr = process.stderr.take()
        .ok_or_else(|| DetailedError::new("stderr was not redirected.".to_string()))?;

    let stdin = process.stdin.as_mut()
        .ok_or_else(|| DetailedError::new("stdin was not redirected.".to_string()))?;

    let stdout_thread : JoinHandle<Result<String, GeneralError>> = thread::spawn(|| {

        let buff = BufReader::new(stdout);

        let mut result = String::new();

        for line_result in buff.lines() {

            let line = line_result?;
            result.push_str(&format!("{}\n", line));
            write_out(&format!("OUT | {}\n", line));
        }

        Ok(result)
    });

    let stderr_thread : JoinHandle<Result<String, GeneralError>> = thread::spawn(|| {

        let buff = BufReader::new(stderr);

        let mut result = String::new();

        for line_result in buff.lines() {

            let line = line_result?;
            result.push_str(&format!("{}\n", line));
            write_err(&format!("ERR | {}\n", line));
        }

        Ok(result)
    });

    stdin.write_all("set -exu\n".as_bytes())?;
    stdin.write_all(format!("{}\n", command).as_bytes())?;
    stdin.write_all("exit $?;\n".as_bytes())?;

    let out_result = stdout_thread.join()??;
    let err_result = stderr_thread.join()??;

    let exit_status = process.wait()?;

    return Ok(CommandResult {
        status_code: exit_status.code(),
        success: exit_status.success(),
        stdout: out_result,
        stderr: err_result,
        command: command.to_string()
    });
}

fn write_out(text: &str){

    print!("{}", text);
}

fn write_err(text: &str) {

    eprint!("{}", text);
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
    pub fn as_result(self) -> Result<CommandResult, DetailedError> {
        if self.success {
            Ok(self)
        } else {
            Err(DetailedError::new(format!(
                "A command exited with a non 0 exit code or with a signal. '{}'",
                self.command
            )))
        }
    }
}
