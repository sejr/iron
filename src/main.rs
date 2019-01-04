extern crate clap;
extern crate pest;
extern crate colored;

#[macro_use]
extern crate pest_derive;

mod util;
mod parser;

use clap::{ App, Arg };
use std::time::Instant;

fn main() {
    let matches = App::new("Forge")
        .version("0.0.1")
        .author("Samuel Roth <sam@roth.fyi>")
        .about("Iron language toolkit")
        .arg(Arg::with_name("info")
            .multiple(true)
            .help("Display information for provided module")
            .short("i")
            .long("info"))
        .arg(Arg::with_name("module")
            .help("Iron module to execute; must have an entry point")
            .required(true))
        .get_matches();

    match matches.value_of("module") {
        Some(file) => {
            let parsing_start = Instant::now();
            if let Some(module) = parser::parse(file) {
                let parsing_complete = Instant::now();
                let parse_time = parsing_complete.duration_since(parsing_start);
                match matches.occurrences_of("info") {
                    1 => util::profile(parse_time, module),
                    2 => println!("{:?}", module),
                    _ => ()
                }
            } else {
                panic!("Error parsing module.");
            }
        },
        _ => unreachable!()
    }
}
