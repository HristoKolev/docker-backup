#![feature(await_macro, async_await, futures_api)]

extern crate tokio;

use tokio::await;
use shiplift::Docker;
use std::env;

fn main() {
    tokio::run_async(main_async());
}

fn exit_with_error(printable: &str) -> ! {
    println!("{}", printable);
    std::process::exit(1);
}

async fn main_async () {

    let args: Vec<String> = env::args().skip(1).collect();

    let volume_name;

    match args.first() {
        Some(res) => volume_name = res,
        None => {
            exit_with_error("You must provide a volume name.");
        }
    }

    println!("{}", volume_name);

    let docker = Docker::host("http://dev-host.lan:2376".parse().unwrap());

    let containers : Vec<shiplift::rep::Container>;

    match await!(docker.containers().list(&Default::default())) {
        Ok(res) => containers = res,
        Err(_) => {
            exit_with_error("An error occurred while listing the containers.");
        }
    }

    let container ;

    match containers.first() {
        Ok(res) => container = res,
        Err(_) => exit_with_error(format!("No container found for name `{}`", cont))
    }

    println!("{:#?}", containers.first().unwrap());
}