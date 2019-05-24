use std::path::{Path, PathBuf};

use super::prelude::*;

pub trait StrExtensions {
    fn last_index_of(&self, c: char) -> Option<usize>;
}

impl StrExtensions for &str {
    fn last_index_of(&self, c: char) -> Option<usize> {

        let mut i = self.len() - 1;

        for x in self.chars().rev() {

            if x == c {
                return Some(i);
            }

            if i > 0 {
                i -= 1;
            }
        }

        None
    }
}

pub trait PathExtensions {
    fn get_as_string(&self) -> Result<String>;
    fn extension_as_string(&self) -> Result<String>;
    fn file_stem_as_string(&self) -> Result<String>;
    fn get_directory_as_string(&self) -> Result<String>;
    fn get_directory(&self) -> PathBuf;
    fn combine_with(&self, p: &str) -> PathBuf;
}

impl PathExtensions for Path {

    fn get_as_string(&self) -> Result<String> {
        Ok(self.to_str()
            .ok_or_else(|| CustomError::from_message("The Path cannot be converted to &str because it is not valid."))?
            .to_string())
    }

    fn extension_as_string(&self) -> Result<String> {

        Ok(self.extension()
            .ok_or_else(|| CustomError::from_message(
                "The file does not have an extension"
            ))?.to_str()
            .ok_or_else(|| CustomError::from_message(
                "The `Path.extension()` OsStr cannot be converted to &str because it is not valid."
            ))?.to_string())
    }

    fn file_stem_as_string(&self) -> Result<String> {

        Ok(self.file_stem()
            .ok_or_else(|| CustomError::from_message(
                "The file does not have a `file_stem`."
            ))?.to_str()
            .ok_or_else(|| CustomError::from_message(
                "The `Path.file_stem()` OsStr cannot be converted to &str because it is not valid."
            ))?.to_string())
    }

    fn get_directory_as_string(&self) -> Result<String> {

        let mut copy = self.clone().to_owned();

        copy.pop();

        copy.get_as_string()
    }

    fn get_directory(&self) -> PathBuf {

        let mut copy = self.clone().to_owned();

        copy.pop();

        copy
    }

    fn combine_with(&self, p: &str) -> PathBuf {

        let mut copy = self.clone().to_owned();

        copy.push(p);

        copy
    }
}
