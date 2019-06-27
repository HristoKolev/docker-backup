use crate::global::prelude::*;
use crate::global::{app_config};
use roxmltree::Document;

pub fn create_kvm_machine_archive(config_name: &str, work_path: &str) -> Result {

    let config = app_config().kvm_machine_config.as_ref()
        .and_then(|x| x.get(config_name).cloned())
        .ok_or_else(|| CustomError::from_message("`KvmMachine` archiving is not configured."))?;

    let vm_xml = bash_exec!("virsh dumpxml {}", config.vm_name).stdout;

    let doc = Document::parse(&vm_xml)?;

    let devices =  doc.descendants()
        .filter(|x| x.has_tag_name("devices"))
        .first(|_| true)
        .ok_or_else(|| CustomError::from_message("No `devices` tag found in the xml definition of the vm."))?;

    let disk = devices.descendants()
        .filter(|x| x.has_tag_name("disk"))
        .filter(|x| x.children().filter(|y| {
            y.has_tag_name("target") && y.attribute("dev") == Some(&config.device_name)
        }).any())
        .first(|_| true)
        .ok_or_else(|| CustomError::from_message("No `disk` tag found in the xml definition of the vm with the correct device name."))?;

    let disk_path = disk.children()
        .filter(|x| x.has_tag_name("source"))
        .map_result(|x| x.attribute("file")
            .ok_or_else(|| CustomError::from_message("No `file` attribute found on the `source` tag in the `disk` tag in the xml definition of the vm."))
        )?
        .first(|_| true)
        .ok_or_else(|| CustomError::from_message("No `source` tag found in the `disk` tag in the xml definition of the vm."))?;


    log!("The path to the disk is `{}`", &disk_path);

    Ok(())
}

pub fn restore_kvm_machine_archive(config_name: &str, _work_path: &str, compressed: &str) -> Result {

    let config = app_config().kvm_machine_config.as_ref()
        .and_then(|x| x.get(config_name).cloned())
        .ok_or_else(|| CustomError::from_message("`KvmMachine` archiving is not configured."))?;



    Ok(())
}
