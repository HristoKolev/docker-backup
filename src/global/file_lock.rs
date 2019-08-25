use std::time::Duration;
use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;

use libc::{flock, LOCK_EX, LOCK_NB, EWOULDBLOCK, __errno_location};

use super::prelude::*;

#[allow(unsafe_code)]
#[allow(unused)]
fn lock_file(file_path: &str) -> Result<Option<File>> {

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)?;

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
pub fn wait_for_lock(file_path: &str) -> Result<File> {
    loop {
        if let Some(file) = lock_file(file_path)? {
            return Ok(file);
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}
