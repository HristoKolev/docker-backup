use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::thread;

fn main() {

    let res = exec("echo 123");
}

fn exec(command: &str) -> CommandResult {

    let mut process = Command::new("/bin/bash")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to spawn a process.");

    let mut stdout = process.stdout.take().unwrap();
    let mut stderr = process.stderr.take().unwrap();
    let mut stdin = process.stdin.as_mut().unwrap();

    // let mut stdout_result = String::new();
    // let mut stderr_result= String::new();

    let std_out_thread= thread::spawn(move || {

        let stdout_buff = BufReader::new(stdout);

        // let mut std_out_vec = String::new();

        for line in stdout_buff.lines() {
            let formatted = format!("OUT | {}", line.unwrap());
            /// std_out_vec.push_str(&formatted);
            write_to_out(&formatted);
        }

        // stdout_result = std_out_vec;
    });

    let std_err_thread= thread::spawn(move || {

        let stderr_buff = BufReader::new(stderr);

        // let mut std_err_vec = String::new();

        for line in stderr_buff.lines() {
            let formatted = format!("ERR | {}", line.unwrap());
            //  std_err_vec.push_str(&formatted);
            write_to_err(&formatted);
        }

        // stderr_result = std_err_vec;
    });

    stdin.write_all(format!("{}\n", command).as_bytes()).unwrap();
    stdin.write_all("exit $?;\n".as_bytes()).unwrap();

    std_out_thread.join().unwrap();
    std_err_thread.join().unwrap();

    let exit_status = process.wait().unwrap();

    return CommandResult {
        status: exit_status.code().unwrap(),
        // stdout: stdout_result,
        // stderr: stderr_result,
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
    // stdout: String,
    // stderr: String,
}