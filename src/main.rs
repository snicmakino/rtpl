#[macro_use]
extern crate clap;

use clap::{App, Arg};

mod input;
mod core;

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::from_usage("-f --file <FILE> 'file path about template setting'"))
        .arg(Arg::from_usage("[TEMPLATE] 'template file path'"));
    // TODO value option
    // TODO env mode option

    let matches = app.get_matches();

    if let Some(file) = matches.value_of("file") {
        match input::read_from_file(file) {
            Ok(v) => println!("{}", v),
            Err(_) => return,
        };
    }

//    match input::read_from_file("Cargo.toml") {
//        Ok(v) => println!("{}", v),
//        Err(_) => return,
//    };
}
