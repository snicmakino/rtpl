#[macro_use]
extern crate clap;

use std::error::Error;
use std::fmt;

use clap::{App, Arg, ArgMatches};
use serde_json::Value;

mod core;
mod input;
mod setting;

// TODO move error for specific module
#[derive(Debug)]
struct SettingNotFoundError;

impl fmt::Display for SettingNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Setting file not found")
    }
}

impl Error for SettingNotFoundError {}


fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::from_usage("-f --file <FILE> 'file path about template setting'"))
        .arg(Arg::from_usage("-t --template [TEMPLATE] 'template file path'"))
        .arg(Arg::from_usage("template [TEMPLATE] 'template file path'"));
    // TODO value option
    // TODO env mode option

    let matches = app.get_matches();

    let setting = setting(&matches).unwrap();
    let template = template(&matches).unwrap();

    print!("{}", core::render(&template, setting));
}

fn setting(matches: &ArgMatches) -> Result<Value, Box<dyn Error>> {
    if let Some(file) = matches.value_of("file") {
        return setting::get_from_file(file);
    }
    return Err(Box::new(SettingNotFoundError));
}

fn template(matches: &ArgMatches) -> Result<String, Box<dyn Error>> {
    return match matches.value_of("template") {
        Some(file) => input::read_from_file(file),
        None => input::read_from_stdin()
    };
}
