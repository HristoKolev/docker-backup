#![feature(await_macro, async_await, futures_api)]

extern crate tokio;

use tokio::await;
use shiplift::Docker;
use std::env;

fn main() {
    tokio::run_async(main_async());
}

async fn main_async () {

    let args: Vec<String> = env::args().skip(1).collect();

    let container_name;

    match args.first() {
        Some(res) => container_name = res,
        None => {
            println!("You must provide a container name.");
            std::process::exit(1);
        }
    }

    let docker = Docker::new();

    let containers = docker.containers();

    let d6 = containers.get(container_name);

    let res =  await!(d6.start());

    match res {
        Ok(res) => println!("OK: {:?}", res),
        Err(err) => println!("ERR: {:?}", err)
    }
}