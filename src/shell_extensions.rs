use std::process::{Command, Stdio};
use std::thread::JoinHandle;
use std::thread;
use std::io::{BufReader, Write, BufRead};
use crate::errors::GeneralError;

pub fn exec(command: &str) -> Result<CommandResult, GeneralError> {

    let mut process = Command::new("/bin/bash")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;

    let stdout = process.stdout.take().unwrap();
    let stderr = process.stderr.take().unwrap();
    let stdin = process.stdin.as_mut().unwrap();

    let stdout_thread : JoinHandle<Result<String, GeneralError>> = thread::spawn(|| {

        let buff = BufReader::new(stdout);

        let mut result = String::new();

        for line_result in buff.lines() {

            let line = line_result?;
            let formatted = format!("OUT | {}\n", line);
            result.push_str(&formatted);
            write_to_out(&formatted);
        }

        Ok(result)
    });

    let stderr_thread : JoinHandle<Result<String, GeneralError>> = thread::spawn(|| {

        let buff = BufReader::new(stderr);

        let mut result = String::new();

        for line_result in buff.lines() {

            let line = line_result?;
            let formatted = format!("ERR | {}\n", line);
            result.push_str(&formatted);
            write_to_err(&formatted);
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
    });
}

fn write_to_out (text: &str){
    print!("{}", text);
}

fn write_to_err(text: &str) {
    eprint!("{}", text);
}

#[derive(Debug)]
pub struct CommandResult {
    status_code: Option<i32>,
    stdout: String,
    stderr: String,
    success: bool,
}