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

    let docker_containers = docker.containers();
    
    let container_list : Vec<shiplift::rep::Container> = await!(
        docker_containers.list(&Default::default())
    ).expect("There was an error while listing the containers.");

    for container_rep in container_list {

        let container = docker_containers.get(&container_rep.id);

        let inspect = await!(container.inspect());

        //let inspect = await!(container_rep.inspect());
        println!("container -> {:#?}", inspect);
    }
}