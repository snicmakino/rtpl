#[macro_use]
extern crate clap;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;

use clap::{App, Arg, ArgGroup, ArgMatches};
use handlebars::Handlebars;
use serde_json::{json, Map, Value};

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
        .arg(Arg::from_usage("-s --setting [FILE] 'setting file'"))
        .arg(Arg::from_usage("-v --value... [VALUE] 'setting values'"))
        .arg(Arg::from_usage("-e --env 'setting env mode'"))
        .group(ArgGroup::with_name("settings")
            .args(&["setting", "value", "env"])
            .required(true))
        .arg(Arg::from_usage("-t --template [TEMPLATE] 'template file'"))
        .arg(Arg::with_name("template file").value_name("TEMPLATE").help("template file'"))
        .arg(Arg::from_usage("-o --output [OUTPUT] 'output file'"));

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
    if let Some(values) = matches.values_of("value") {
        let mut map = Map::new();
        for value in values {
            let v: Vec<&str> = value.splitn(2, "=").collect();
            map.insert(v[0].to_string(), v[1].into());
        }
        return Ok(map.into());
    }
    if matches.is_present("env") {
        let vars: HashMap<String, String> = std::env::vars().collect();
        return Ok(json!(vars));
    }
    return Err(Box::new(SettingNotFoundError));
}

fn template(matches: &ArgMatches) -> Result<String, Box<dyn Error>> {
    if let Some(file) = matches.value_of("template") {
        return input::read_from_file(file);
    }
    if let Some(file) = matches.value_of("template file") {
        return input::read_from_file(file);
    }
    return input::read_from_stdin();
}

fn output(matches: &ArgMatches, content: String) {
    match matches.value_of("output") {
        Some(file) => output::to_file(content, file),
        None => output::to_stdout(content)
    };
}
