use clap::{App, ArgMatches};

pub fn command_config<F>(command_name: &str, f: F) -> ArgMatches
    where F: for<'a, 'b> FnOnce(App<'a, 'b>) -> App<'a, 'b> {

    let mut matches = App::new(format!("XDXD Backup - {}", command_name))
        .version("1.0")
        .author("Hristo Kolev")
        .about("Backs things up.");

    matches = f(matches);

    matches.get_matches()
}
