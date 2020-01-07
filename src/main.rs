#[macro_use]
extern crate clap;

use std::error::Error;
use std::fmt;

use clap::{App, Arg, ArgMatches};
use handlebars::Handlebars;
use serde_json::Value;

mod input;
mod output;
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
        .arg(Arg::from_usage("-s --setting <FILE> 'setting file'"))
        .arg(Arg::from_usage("-t --template [TEMPLATE] 'template file'"))
        .arg(Arg::from_usage("-o --output [OUTPUT] 'output file'"));
    // TODO value option -v --values
    // TODO env mode option -e --env

    let matches = app.get_matches();

    let setting = setting(&matches).unwrap();
    let template = template(&matches).unwrap();

    let handlebars = Handlebars::new();
    let result = handlebars.render_template(&template, &setting).unwrap();
    output(&matches, result);
}

fn setting(matches: &ArgMatches) -> Result<Value, Box<dyn Error>> {
    if let Some(file) = matches.value_of("setting") {
        return setting::get_from_file(file);
    }
    return Err(Box::new(SettingNotFoundError));
}

fn template(matches: &ArgMatches) -> Result<String, Box<dyn Error>> {
    // TODO Consider whether standard input is required
    return match matches.value_of("template") {
        Some(file) => input::read_from_file(file),
        None => input::read_from_stdin()
    };
}

fn output(matches: &ArgMatches, content: String) {
    match matches.value_of("output") {
        Some(file) => output::to_file(content, file),
        None => output::to_stdout(content)
    };
}
