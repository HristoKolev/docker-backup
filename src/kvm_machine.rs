use std::path::Path;
use roxmltree::Document;

use crate::global::prelude::*;

static VM_XML_DEFINITION_FILE_NAME: &str = "machine.xml";
static SNAPSHOT_NAME: &str = "backup-snapshot";

#[derive(Debug, Clone)]
pub struct DiskImage {
    pub device_name: String,
    pub file_path: String,
    pub disk_format: String,
    pub disk_device: String,
}

fn get_disks(xml: &str) -> Result<Vec<DiskImage>> {

    let doc = Document::parse(xml)?;

    let mut results = Vec::new();

    let disk_nodes = doc.descendants()
        .filter_first(|x| x.has_tag_name("devices"))
        .or_error("No `devices` tag found in the xml definition of the vm.")?
        .children().filter(|x| x.has_tag_name("disk"));

    for disk_node in disk_nodes {

        let disk_device = disk_node.attribute("device")
            .or_error("No `device` attribute found on the `disk` tag.")?
            .to_string();

        let driver_node = disk_node.children()
            .filter_first(|x| x.has_tag_name("driver"))
            .or_error("No `driver` tag found in the `disk` tag.")?;

        let disk_format = driver_node.attribute("type")
            .or_error("No `type` attribute found on the `driver` tag.")?
            .to_string();

        let target_node = disk_node.children()
            .filter_first(|x| x.has_tag_name("target"))
            .or_error("No `target` tag found in the `disk` tag.")?;

        let device_name = target_node.attribute("dev")
            .or_error("No `dev` attribute found on the `target` tag.")?
            .to_string();

        let source_node = disk_node.children()
            .filter_first(|x| x.has_tag_name("source"))
            .or_error("No `source` tag found in the `disk` tag.")?;

        let file_path = source_node.attribute("file")
            .or_error("No `file` attribute found on the `source` tag.")?
            .to_string();

        results.push(DiskImage {
            device_name,
            file_path,
            disk_format,
            disk_device,
        });
    }

    Ok(results)
}

pub fn create_kvm_machine_archive(config_name: &str, work_path: &str) -> Result {

    let config = app_config().kvm_machine_config.as_ref()
        .and_then(|x| x.get(config_name).cloned())
        .or_error("`KvmMachine` archiving is not configured.")?;

    let vm_xml = bash_exec_no_log!("virsh dumpxml {}", &config.vm_name).stdout;

    let disks = get_disks(&vm_xml)?;

    let xml_file_name = Path::new(work_path)
        .join(VM_XML_DEFINITION_FILE_NAME).get_as_string()?;

    let available_disks = disks
        .into_iter()
        .filter(|x| x.disk_format == "qcow2")
        .collect_vec();

    let selected_disks = (&available_disks).into_iter()
        .filter(|x| config.device_names.contains(&x.device_name))
        .map(|x| x.clone())
        .collect_vec();

    do_try::run(|| {

        bash_exec!(
            r#"virsh snapshot-create-as --domain {} {} --disk-only --atomic"#,
            &config.vm_name,
            SNAPSHOT_NAME
        );

        ::std::fs::write(&xml_file_name, vm_xml)?;

        for disk in &selected_disks {

            let new_disk_path = Path::new(work_path)
                .get_directory()
                .join(Path::new(&disk.file_path).file_name_as_string()?)
                .get_as_string()?;

            bash_exec!("qemu-img convert -O qcow2 {} {}", &disk.file_path, &new_disk_path);

            bash_exec!("qemu-img check {}", &new_disk_path);
        }

        Ok(())

    }).finally(|| {

        for disk in &available_disks {

            bash_exec!("virsh blockcommit {} {} --active --pivot", &config.vm_name, &disk.device_name);

            let snapshot_disk_path = Path::new(&disk.file_path)
                .change_extension(SNAPSHOT_NAME)?
                .get_as_string()?;

            bash_exec!("rm {} -f", &snapshot_disk_path);
        }

        bash_exec!("virsh snapshot-delete {} --metadata {}", &config.vm_name, SNAPSHOT_NAME);

        Ok(())
    })?;

    Ok(())
}

pub fn restore_kvm_machine_archive(_config_name: &str, _work_path: &str, _compressed: &str) -> Result {


    Err(CustomError::from_message("Archive restoration is not supported for type `kvm-machine`. You can unpack it and restore it manually."))
}
