use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, Write};
use std::{thread};
use std::any::Any;
use std::result::Result;

#[macro_use]
extern crate derive_more;

fn main() {

   match exec("echo 123") {
       Ok(res) => {
           println!("{:#?}", res);
       },
       Err(err) => {
           println!("{:#?}", err);
       }
   }
}

fn exec(command: &str) -> Result<CommandResult, GeneralError> {

    let mut process = Command::new("/bin/bash")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;

    let stdout = process.stdout.take().unwrap();
    let stderr = process.stderr.take().unwrap();
    let stdin = process.stdin.as_mut().unwrap();

    let std_out_thread= thread::spawn(move || {

        let buff = BufReader::new(stdout);

        let mut result = String::new();

        for line_result in buff.lines() {
            match line_result {
                Ok(line) => {
                    let formatted = format!("OUT | {}\n", line);
                    result.push_str(&formatted);
                    write_to_out(&formatted);
                },
                Err(err) => return Err(err)
            }
        }

        Ok(result)
    });

    let std_err_thread= thread::spawn(move || {

        let buff = BufReader::new(stderr);

        let mut result = String::new();

        for line_result in buff.lines() {
            match line_result {
                Ok(line) => {
                    let formatted = format!("ERR | {}\n", line);
                    result.push_str(&formatted);
                    write_to_err(&formatted);
                },
                Err(err) => return Err(err)
            }
        }

        Ok(result)
    });

    stdin.write_all("set -exu\n".as_bytes())?;
    stdin.write_all(format!("{}\n", command).as_bytes())?;
    stdin.write_all("exit $?;\n".as_bytes())?;

    let out_result = std_out_thread.join()??;
    let err_result = std_err_thread.join()??;

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
struct CommandResult {
    status_code: Option<i32>,
    stdout: String,
    stderr: String,
    success: bool,
}

#[derive(From, Debug)]
enum GeneralError {
    IoError(std::io::Error),
    Dynamic(Box<dyn Any + Send + 'static>)
}
