use std::time::Duration;
use std::sync::mpsc::channel;
use std::fs::File;
use std::path::Path;
use std::os::unix::io::AsRawFd;
use std::thread::JoinHandle;

use libc::{flock, LOCK_EX, LOCK_NB, EWOULDBLOCK, __errno_location};

use super::prelude::*;


#[allow(unsafe_code)]
#[allow(unused)]
fn lock_file(name: &str) -> Result<Option<File>> {

    let path = Path::new(name);

    let file = if path.exists() {
        File::open(path)?
    } else {
        File::create(path)?
    };

    unsafe {

        let rc = flock(file.as_raw_fd(), LOCK_EX | LOCK_NB);

        let is_locked = rc == 0 || EWOULDBLOCK != *__errno_location();

        if is_locked {
            Ok(Some(file))
        } else {
            Ok(None)
        }
    }
}

#[allow(unused)]
pub fn wait_for_lock(file_name: &str) -> Result<File> {

    if let Some(file) = lock_file(file_name)? {
        return Ok(file);
    }

    let (tx, rx) = channel();

    let t_file_name = file_name.to_string();

    let prod_thread: JoinHandle<Result> = std::thread::spawn(move || {

        loop {

            if let Some(file) = lock_file(&t_file_name)? {

                tx.send(file)?;
                return Ok(());
            }

            std::thread::sleep(Duration::from_millis(100));
        }
    });

    let file = rx.recv()?;

    prod_thread.join()
        .on_error("The polling thread died for some reason.")??;

    Ok(file)
}
