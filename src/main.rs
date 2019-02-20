#![feature(await_macro, async_await, futures_api)]

extern crate tokio;

use tokio::await;
use shiplift::Docker;
use shiplift::rep::Volume as VolumeRep;
use shiplift::rep::Container as ContainerRep;
use shiplift::rep::ContainerDetails as ContainerDetailsRep;

fn main() {

    tokio::run_async(main_async());
}

async fn main_async () {

    let volume_name = get_volume_name();

    let docker = create_client();

    let volume = await!(get_volume_by_name(&docker, &volume_name));

    let container_details = await!(get_container_details(&docker));

    //let connected_containers = container_details.into_iter().filter()

    println!("{:#?}", container_details);

    // println!("{:#?}", volume_mountpoints);

//    let container_list = await!(
//        docker.containers().list(&Default::default())
//    ).expect("There was an error while listing the containers.");
//
//
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

    let volume_mountpoints : Vec<_> = await!(
        docker.volumes().list()
    ).expect("There was an error while listing the volumes.");

    let volume: VolumeRep = volume_mountpoints
        .into_iter()
        .filter(|x| x.name == volume_name)
        .nth(0)
        .expect(&format!("No volume found for name `{}`", volume_name));

    volume
}

async fn get_container_details(docker: &Docker) -> Vec<ContainerDetailsRep> {

    let containers_rep: Vec<ContainerRep> = await!(
        docker.containers()
            .list(&Default::default())
    ).expect("There was an error while listing the containers.");

    let mut connected_containers = Vec::new();

    for container_rep in containers_rep {

        let container = docker.containers().get(&container_rep.id);

        let detailed_container = await!(container.inspect())
            .expect(&format!("There was an error while getting details for container {}", container_rep.id));

        connected_containers.push(detailed_container);
    }

    connected_containers
}