use std::path::Path;

use roxmltree::Document;

use crate::global::prelude::*;

static VM_XML_DEFINITION_FILE_NAME: &str = "machine.xml";
static SNAPSHOT_NAME: &str = "backup-snapshot";
static SNAPSHOT_FILE_NAME: &str = "snapshot.qcow2";
static IMAGE_FILE_NAME: &str = "image.qcow2";

pub fn create_kvm_machine_archive(config_name: &str, work_path: &str) -> Result {

    let config = app_config().kvm_machine_config.as_ref()
        .and_then(|x| x.get(config_name).cloned())
        .or_error("`KvmMachine` archiving is not configured.")?;

    let vm_xml = bash_exec_no_log!("virsh dumpxml {}", &config.vm_name).stdout;

    let disk_path = get_disk_path(&vm_xml, &config.device_name)?;

    let snapshot_path = Path::new(work_path)
        .join(SNAPSHOT_FILE_NAME).get_as_string()?;

    let image_file_name = Path::new(work_path)
        .join(IMAGE_FILE_NAME).get_as_string()?;

    let xml_file_name = Path::new(work_path)
        .join(VM_XML_DEFINITION_FILE_NAME).get_as_string()?;

    do_try::run(|| {

        bash_exec!("virsh dumpxml {} > {}", &config.vm_name, &xml_file_name);

        bash_exec!(
            r#"virsh snapshot-create-as --domain {} {} --diskspec "{},file={}" --disk-only --atomic"#,
            &config.vm_name,
            SNAPSHOT_NAME,
            &config.device_name,
            &snapshot_path
        );

        bash_exec!("qemu-img convert -O qcow2 {} {}", &disk_path, image_file_name);
        bash_exec!("qemu-img check {}", image_file_name);

        Ok(())

    }).finally(|| {

        do_try::run(|| {

            bash_exec!("virsh blockcommit {} {} --active --pivot", &config.vm_name, &config.device_name);

            Ok(())
        }).finally(|| {

            bash_exec!("virsh snapshot-delete {} --metadata {}", &config.vm_name, SNAPSHOT_NAME);
            bash_exec!("rm {} -f", &snapshot_path);

            Ok(())
        })?;

        Ok(())
    })?;

    Ok(())
}

fn get_disk_path(xml: &str, device_name: &str) -> Result<String> {

    let doc = Document::parse(xml)?;

    let devices =  doc.descendants()
        .filter_first(|x| x.has_tag_name("devices"))
        .or_error("No `devices` tag found in the xml definition of the vm.")?;

    let disk = devices.descendants()
        .filter(|x| x.has_tag_name("disk"))
        .filter_first(|x| x.children().filter(|y| {
            y.has_tag_name("target") && y.attribute("dev") == Some(device_name)
        }).has_any())
        .or_error("No `disk` tag found in the xml definition of the vm with the correct device name.")?;

    let disk_path = disk.children()
        .filter(|x| x.has_tag_name("source"))
        .map_result(|x| x.attribute("file")
            .or_error("No `file` attribute found on the `source` tag in the `disk` tag in the xml definition of the vm.")
        )?
        .first()
        .or_error("No `source` tag found in the `disk` tag in the xml definition of the vm.")?;

    Ok(disk_path.to_string())
}

pub fn restore_kvm_machine_archive(config_name: &str, _work_path: &str, compressed: &str) -> Result {

    let config = app_config().kvm_machine_config.as_ref()
        .and_then(|x| x.get(config_name).cloned())
        .or_error("`KvmMachine` archiving is not configured.")?;

    let temp_xml_file_path = format!("/tmp/{}.xml", ::uuid::Uuid::new_v4().to_string());

    bash_exec!(
        "unrar p -inul {} {} > {}",
        &compressed,
        IMAGE_FILE_NAME,
        &config.restore_image_path
    );

    bash_exec!("unrar p -inul {} {} > {}",
        &compressed,
        VM_XML_DEFINITION_FILE_NAME,
        &temp_xml_file_path
    );

    bash_exec!("virsh define {}", &temp_xml_file_path);

    bash_exec!("rm {} -f", &temp_xml_file_path);

    Ok(())
}
