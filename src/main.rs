use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::thread;

fn main() {

    let mut process = Command::new("/bin/bash")
        .arg("./cats.sh")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn a process.");

    let stdout_buff = BufReader::new(process.stdout.take().unwrap());
    let stderr_buff = BufReader::new(process.stderr.take().unwrap());

    let std_out_thread= thread::spawn(move || {
        stdout_buff.lines().for_each(|line|
            println!("out: {}", line.unwrap())
        );
    });

    let std_err_thread= thread::spawn(move || {
        stderr_buff.lines().for_each(|line|
            println!("err: {}", line.unwrap())
        );
    });

    std_out_thread.join().unwrap();
    std_err_thread.join().unwrap();

    let status = process.wait().unwrap();
    println!("{}", status);
}


