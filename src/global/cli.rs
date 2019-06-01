use clap::{App, ArgMatches, Arg};
use std::ffi::OsString;

use crate::global::prelude::*;
use std::sync::Mutex;
use std::collections::HashMap;

pub struct CliRunner {
    pub command_map: Mutex<HashMap<String, Box<(Fn() -> Result + Send + Sync)>>>,
}

impl CliRunner {
    pub fn new() -> CliRunner {
        CliRunner {
            command_map: Mutex::new(HashMap::new())
        }
    }

    pub fn command_config<F>(&self, command_name: &str, f: F) -> ArgMatches
        where F: for<'a, 'b> FnOnce(App<'a, 'b>) -> App<'a, 'b> {

        let mut matches = App::new(format!("XDXD Backup - {}", command_name))
            .version("1.0")
            .author("Hristo Kolev")
            .about("Backs things up.");

        matches = f(matches);

        let mut i = 0;
        let args = ::std::env::args_os().filter(|x| {

            let result = i != 1;

            i += 1;

            result
        }).collect::<Vec<OsString>>();

        matches.get_matches_from(args)
    }

    pub fn register_command<F>(&self, command_name: &str, func: F) -> Result
        where F: Fn() -> Result + Send + Sync, F: 'static {

        let mut map = self.command_map.lock()?;
        map.insert(command_name.to_string(), Box::new(func));

        Ok(())
    }

    pub fn run(&self) -> Result{

        let command_name = ::std::env::args()
            .skip(1).take(1)
            .collect::<Vec<String>>().get(0)
            .map(|x| x.to_string());

        if let Some(command_name) = command_name {

        } else {
            super::logger().log()?;
        }

    }
}

