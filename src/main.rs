#![feature(await_macro, async_await, futures_api)]

extern crate tokio;

use tokio::await;

use shiplift::Docker;

use shiplift::rep::Container as ContainerRep;
use shiplift::rep::ContainerDetails as ContainerDetailsRep;
use shiplift::rep::Volume as VolumeRep;

fn main() {

    tokio::run_async(main_async());
}

#[derive(Debug)]
struct Cat {
    age: i32,
    name: String,
}

impl Cat {
    pub fn new() -> Cat {
        Cat{
            name: "Zay".to_string(),
            age: 2
        }
    }
}


async fn main_async () {

    let cat = Cat {
        age: 1,
        name: "Keks".to_string()
    };

    println!("{:#?}", cat);



    let volume_name = get_volume_name();

    let docker = create_client();

    let volume = await!(get_volume_by_name(&docker, &volume_name));

    let container_details = await!(get_connected_containers(&docker, &volume));

    println!("{:#?}", container_details);
}

fn get_volume_name() -> String {

    let volume_name = std::env::args()
        .nth(1)
        .expect("A first argument must be specified. (volume name)");

    volume_name
}

fn create_client () -> Docker {

    let uri = "http://dev-host.lan:2376".parse().unwrap();
    let docker = Docker::host(uri);

    docker
}

async fn get_volume_by_name<'a> (docker: &'a Docker, volume_name: &'a str) -> VolumeRep {

    let volume_mountpoints : Vec<_> = await!(docker.volumes().list())
        .expect("There was an error while listing the volumes.");

    let volume: VolumeRep = volume_mountpoints
        .into_iter()
        .filter(|x| x.name == volume_name)
        .nth(0)
        .expect(&format!("No volume found for name `{}`.", volume_name));

    volume
}

async fn get_connected_containers<'a>(docker: &'a Docker, volume: &'a VolumeRep) -> Vec<ContainerDetailsRep> {

    let container_reps: Vec<ContainerRep> = await!(docker.containers().list(&Default::default()))
        .expect("There was an error while listing the containers.");

    let mut connected_containers = Vec::new();

    for container_rep in container_reps {

        let container = docker.containers().get(&container_rep.id);
        let container_detail = await!(container.inspect())
            .expect(&format!("There was an error while getting details for container `{}`.", container_rep.id));

        for mount in &container_detail.mounts {
            if mount.source == volume.mountpoint {
                connected_containers.push(container_detail.clone());
            }
        }
    }

    connected_containers
}
