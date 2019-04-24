use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, Write};
use std::thread;

fn main() {

    let res = exec("echo 123");

    println!("{:#?}", res);
}

fn exec(command: &str) -> CommandResult {

    let mut process = Command::new("/bin/bash")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to spawn a process.");

    let stdout = process.stdout.take().unwrap();
    let stderr = process.stderr.take().unwrap();
    let stdin = process.stdin.as_mut().unwrap();

    let std_out_thread= thread::spawn(move || {

        let buff = BufReader::new(stdout);

        let mut result = String::new();

        for line in buff.lines() {
            let formatted = format!("OUT | {}\n", line.unwrap());
            result.push_str(&formatted);
            write_to_out(&formatted);
        }

        result
    });

    let std_err_thread= thread::spawn(move || {

        let buff = BufReader::new(stderr);

        let mut result = String::new();

        for line in buff.lines() {
            let formatted = format!("ERR | {}\n", line.unwrap());
            result.push_str(&formatted);
            write_to_err(&formatted);
        }

        result
    });

    stdin.write_all("set -exu\n".as_bytes()).unwrap();
    stdin.write_all(format!("{}\n", command).as_bytes()).unwrap();
    stdin.write_all("exit $?;\n".as_bytes()).unwrap();

    let out_result = std_out_thread.join().unwrap();
    let err_result = std_err_thread.join().unwrap();

    let exit_status = process.wait().unwrap();

    return CommandResult {
        status: exit_status.code().unwrap(),
        stdout: out_result,
        stderr: err_result,
    };
}

fn write_to_out (text: &str){
    print!("{}", text);
}

fn write_to_err(text: &str) {
    eprint!("{}", text);
}

#[derive(Debug)]
struct CommandResult {
    status: i32,
    stdout: String,
    stderr: String,
}