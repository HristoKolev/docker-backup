#![feature(await_macro, async_await, futures_api)]

extern crate tokio;

use tokio::await;
use shiplift::Docker;
use std::env;

fn main() {
    tokio::run_async(main_async());
}

fn exit_with_error(message: &str) {
    println!("{}", message);
    std::process::exit(1);
}

async fn main_async () {

    let args: Vec<String> = env::args().skip(1).collect();

    let volume_name;

    match args.first() {
        Some(res) => volume_name = res,
        None => {
            exit_with_error("You must provide a volume name.")
        }
    }

    let docker = Docker::new();

    let containers;


    match await!(docker.containers().list(&Default::default())) {
        Ok(res) => containers = res,
        Err(err) => exit_with_error(err)
    }



    println!("{}", containers);


}