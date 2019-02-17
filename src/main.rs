#![feature(await_macro, async_await, futures_api)]

extern crate tokio;

use tokio::await;
use shiplift::Docker;


fn main() {
    tokio::run_async(main_async());
}

fn exit_with_error(printable: &str) -> ! {
    println!("{}", printable);
    std::process::exit(1);
}

async fn main_async () {

    let docker = Docker::host("http://dev-host.lan:2376".parse().unwrap());

    let volume_mountpoints : Vec<shiplift::rep::Volume> = await!(
        docker.volumes().list()
    ).expect("There was an error while listing the volumes.");

    

//    let container_list = await!(
//        docker.containers().list(&Default::default())
//    ).expect("There was an error while listing the containers.");
//
//    for container_rep in container_list {
//        let container = docker.containers().get(&container_rep.id);
//        let inspect = await!(container.inspect());
//        println!("container -> {:#?}", inspect);
//    }
}