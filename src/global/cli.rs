use clap::{App, ArgMatches};
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

    pub fn command_config<F>(&self, f: F) -> ArgMatches
        where F: for<'a, 'b> FnOnce(App<'a, 'b>) -> App<'a, 'b> {

        let mut matches = App::new(format!("XDXD Backup"))
            .version(env!("CARGO_PKG_VERSION"))
            .author("Hristo Kolev")
            .about("Backs things up.");

        matches = f(matches);

        let mut i = 0;
        let args: Vec<OsString> = ::std::env::args_os().filter(|_| {

            let result = i != 1;

            i += 1;

            result
        }).collect();

        matches.get_matches_from(args)
    }

    pub fn register_command<F>(&self, command_name: &str, func: F) -> Result
        where F: Fn() -> Result + Send + Sync, F: 'static {

        let mut map = self.command_map.lock()?;
        map.insert(command_name.to_string(), Box::new(func));

        Ok(())
    }

    pub fn run(&self) -> Result {

        let command_name = ::std::env::args_os()
            .skip(1).take(1)
            .collect::<Vec<OsString>>().get(0)
            .map(|x| x.get_as_string());

        let command_map = self.command_map.lock()?;

        let available_commands = command_map.iter()
            .map(|(key, _val)| key.to_string())
            .order_by(|x| x.to_string())
            .collect_vec();

        let invalid_command_error = || CustomError::user_error(&format!(
            "Please provide a valid command. Available commands: {}", available_commands.join(", ")
        ));

        let command_name = command_name.map(|x | x.map(|y| y.to_lowercase()))
            .ok_or_else(invalid_command_error)??;

        let command = command_map.get(&command_name)
            .ok_or_else(invalid_command_error)?;

        command()?;

        Ok(())
    }
}

