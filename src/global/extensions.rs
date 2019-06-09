use std::path::{Path, PathBuf};

use super::prelude::*;
use std::ffi::OsStr;

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

pub trait OsStrExtensions {
    fn get_as_string(&self) -> Result<String>;
}

impl OsStrExtensions for OsStr {
    fn get_as_string(&self) -> Result<String> {

        Ok(self.to_str()
            .ok_or_else(|| CustomError::from_message("The OsStr cannot be converted to &str because it is not valid."))
            ?.to_string())
    }
}

pub trait PathExtensions {
    fn get_as_string(&self) -> Result<String>;
    fn extension_as_string(&self) -> Result<String>;
    fn file_stem_as_string(&self) -> Result<String>;
    fn file_name_as_string(&self) -> Result<String>;
    fn get_directory_as_string(&self) -> Result<String>;
    fn get_directory(&self) -> PathBuf;
    fn combine_with(&self, p: &str) -> PathBuf;
    fn create_directory(&self) -> Result<PathBuf>;
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
            ))?.get_as_string()?)
    }

    fn file_stem_as_string(&self) -> Result<String> {

        Ok(self.file_stem()
            .ok_or_else(|| CustomError::from_message(
                "The file does not have a `file_stem`."
            ))?.get_as_string()?)
    }

    fn file_name_as_string(&self) -> Result<String> {

        Ok(self.file_name()
            .ok_or_else(|| CustomError::from_message(
                "The file does not have a `file_stem`."
            ))?.get_as_string()?)
    }

    fn get_directory_as_string(&self) -> Result<String> {

        let mut copy = self.to_path_buf();

        copy.pop();

        copy.get_as_string()
    }

    fn get_directory(&self) -> PathBuf {

        let mut copy = self.to_path_buf();

        copy.pop();

        copy
    }

    fn combine_with(&self, p: &str) -> PathBuf {

        let mut copy = self.to_path_buf();

        copy.push(p);

        copy
    }

    fn create_directory(&self) -> Result<PathBuf> {

        let copy = self.to_path_buf();

        ::std::fs::create_dir_all(copy.get_as_string()?)?;

        Ok(copy)
    }
}

pub trait OptionFlatten<T> {
    fn flatten(self) -> Option<T>;
}

impl<T> OptionFlatten<T> for Option<Option<T>> {
    fn flatten(self) -> Option<T> {
        match self {
            None => None,
            Some(v) => v,
        }
    }
}

impl<T> OptionFlatten<T> for Option<Option<Option<T>>> {
    fn flatten(self) -> Option<T> {
        match self {
            None => None,
            Some(v) => v.flatten(),
        }
    }
}

impl<T> OptionFlatten<T> for Option<Option<Option<Option<T>>>> {
    fn flatten(self) -> Option<T> {
        match self {
            None => None,
            Some(v) => v.flatten(),
        }
    }
}

impl<T> OptionFlatten<T> for Option<Option<Option<Option<Option<T>>>>> {
    fn flatten(self) -> Option<T> {
        match self {
            None => None,
            Some(v) => v.flatten(),
        }
    }
}

pub trait OptionBorrow<T> {
    fn map<U, F: FnOnce(&T) -> U>(&self, f: F) -> Option<U>;
    fn map_result<U, F: FnOnce(&T) -> Result<U>>(&self, f: F) -> Option<Result<U>>;
}

impl<T> OptionBorrow<T> for Option<T> {

    fn map<U, F: FnOnce(&T) -> U>(&self, f: F) -> Option<U> {
        match self {
            Some(x) => Some(f(x)),
            None => None,
        }
    }

    fn map_result<U, F: FnOnce(&T) -> Result<U>>(&self, f: F) -> Option<Result<U>> {
        match self {
            Some(x) => Some(f(x)),
            None => None,
        }
    }
}
