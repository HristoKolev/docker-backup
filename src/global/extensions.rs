use std::path::{Path, PathBuf};
use std::ffi::{OsStr, OsString};
use std::collections::HashMap;
use std::hash::Hash;

use super::prelude::*;

pub trait StringExtensions {
    fn last_index_of(self, c: char) -> Option<usize>;
    fn pad_left(self, pad: usize, c: char) -> String;
    fn pad_right(self, pad: usize, c: char) -> String;
}

impl StringExtensions for &str {

    fn last_index_of(self, c: char) -> Option<usize> {

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

    fn pad_left(self, pad: usize, c: char) -> String {

        let mut result = String::new();

        let len = self.len();

        if pad > len {

            for _ in 0..pad-len {

                result.push(c);
            }
        }

        result.push_str(self);

        result
    }

    fn pad_right(self, pad: usize, c: char) -> String {

        let mut result = String::new();

        let len = self.len();

        result.push_str(self);

        if pad > len {

            for _ in 0..pad-len {

                result.push(c);
            }
        }

        result
    }
}

impl StringExtensions for String {

    fn last_index_of(self, c: char) -> Option<usize> {
        self.as_str().last_index_of(c)
    }

    fn pad_left(self, pad: usize, c: char) -> String {
        self.as_str().pad_left(pad, c)
    }

    fn pad_right(self, pad: usize, c: char) -> String {
        self.as_str().pad_right(pad, c)
    }
}

pub trait OsStringExtensions {

    fn get_as_string(&self) -> Result<String>;
}

impl OsStringExtensions for OsStr {

    fn get_as_string(&self) -> Result<String> {

        Ok(self.to_str()
            .or_error("The OsStr cannot be converted to &str because it is not valid.")
            ?.to_string())
    }
}

impl OsStringExtensions for OsString {

    fn get_as_string(&self) -> Result<String> {

        Ok(self.to_str()
            .or_error("The OsStr cannot be converted to &str because it is not valid.")
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
    fn create_directory(&self) -> Result<PathBuf>;
    fn change_extension(&self, new_extension: &str) -> Result<PathBuf>;
}

impl PathExtensions for Path {

    fn get_as_string(&self) -> Result<String> {
        Ok(self.to_str()
            .or_error("The Path cannot be converted to &str because it is not valid.")?
            .to_string())
    }

    fn extension_as_string(&self) -> Result<String> {

        Ok(self.extension()
            .or_error("The file does not have an extension")?
            .get_as_string()?)
    }

    fn file_stem_as_string(&self) -> Result<String> {

        Ok(self.file_stem()
            .or_error("The file does not have a `file_stem`.")?
            .get_as_string()?)
    }

    fn file_name_as_string(&self) -> Result<String> {

        Ok(self.file_name()
            .or_error("The file does not have a `file_stem`.")?
            .get_as_string()?)
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

    fn create_directory(&self) -> Result<PathBuf> {

        let copy = self.to_path_buf();

        ::std::fs::create_dir_all(copy.get_as_string()?)?;

        Ok(copy)
    }

    fn change_extension(&self, new_extension: &str) -> Result<PathBuf> {

        let mut directory = self.get_directory();

        let file_stem = self.file_stem_as_string()?;

        directory.push(format!("{}.{}", &file_stem, new_extension));

        Ok(directory)
    }
}

pub trait OptionExtensions<T> {
    fn map<U, F: FnOnce(&T) -> U>(&self, f: F) -> Option<U>;
    fn map_result<U, F: FnOnce(&T) -> Result<U>>(&self, f: F) -> Result<Option<U>>;
    fn or_error(self, error_message: &str) -> Result<T>;
}

impl<T> OptionExtensions<T> for Option<T> {

    fn map<U, F: FnOnce(&T) -> U>(&self, f: F) -> Option<U> {
        match self {
            Some(x) => Some(f(x)),
            None => None,
        }
    }

    fn map_result<U, F: FnOnce(&T) -> Result<U>>(&self, f: F) -> Result<Option<U>> {
        Ok(match self {
            Some(x) => Some(f(x)?),
            None => None,
        })
    }

    fn or_error(self, msg: &str) -> Result<T> {

        self.ok_or_else(|| CustomError::from_message(msg))
    }
}

pub trait IteratorExtensions: Iterator {

    fn order_by<K, F>(self, f: F) -> ::std::vec::IntoIter<Self::Item>
        where Self: Sized, K: Ord, F: FnMut(&Self::Item) -> K {

        let mut vec = self.collect_vec();
        vec.sort_by_key(f);
        vec.into_iter()
    }

    fn order_by_desc<K, F>(self, f: F) -> ::std::vec::IntoIter<Self::Item>
        where Self: Sized, K: Ord, F: FnMut(&Self::Item) -> K {

        let mut vec = self.collect_vec();
        vec.sort_by_key(f);
        vec.reverse();
        vec.into_iter()
    }

    fn group_by<K, F>(self, f: F) -> ::std::collections::hash_map::IntoIter<K, Vec<Self::Item>>
        where Self: Sized, K: Eq, K: Hash, F: Fn(&Self::Item) -> K {

        let mut group_map = HashMap::new();

        for item in self {

            let value = group_map
                .entry(f(&item))
                .or_insert(Vec::new());

            value.push(item);
        }

        group_map.into_iter()
    }

    fn filter_first<F>(self, f: F) -> Option<Self::Item>
        where Self: Sized, Self::Item: Clone, F: Fn(&Self::Item) -> bool {

        let vec = self.filter(f).take(1).collect_vec();

        vec.first().map(|x| x.clone())
    }

    fn first(self) -> Option<Self::Item>
        where Self: Sized, Self::Item: Clone {

        let vec = self.take(1).collect_vec();

        vec.first().map(|x| x.clone())
    }

    fn has_any(&mut self) -> bool {

        self.next().is_some()
    }

    fn any_result<F>(self, f: F) -> Result<bool>
        where Self: Sized, F: Fn(&Self::Item) -> Result<bool> {

        for item in self {

            if f(&item)? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn map_result<K, F>(self, f: F) -> Result<::std::vec::IntoIter<K>>
        where Self: Sized, F: Fn(&Self::Item) -> Result<K> {

        let source = self.collect_vec();

        let mut destination = Vec::new();

        for item in source {

            destination.push(f(&item)?);
        }

        Ok(destination.into_iter())
    }

    fn collect_vec(self) -> Vec<Self::Item> where Self: Sized {

        self.collect()
    }
}

impl<T> IteratorExtensions for T where T: Iterator { }
