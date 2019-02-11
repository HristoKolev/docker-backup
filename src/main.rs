use shiplift::Docker;
use tokio::prelude::Future;
use std::time::Duration;

fn main() {

    let docker = Docker::new();

    let containers = docker.containers();

    let d6 = containers.get("kfc_app_1");

    d6.stop(Some(Duration::from_secs(10))).wait().unwrap();


//      let containers = docker
//        .containers()
//          .list((&Default::default()))
//            .map(|containers| {
//                for c in containers {
//                    println!("container -> {}", c.names.first().unwrap())
//                }
//            })
//            .map_err(|e| eprintln!("Error: {}", e)).wait().unwrap();


}