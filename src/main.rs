extern crate clap;
extern crate pest;

#[macro_use]
extern crate pest_derive;

mod parser;
use clap::{ App, Arg };

fn main() {
    let matches = App::new("Forge")
        .version("0.0.1")
        .author("Samuel Roth <sam@roth.fyi>")
        .about("Iron language toolkit")
        .arg(Arg::with_name("INPUT")
            .help("Iron file to execute")
            .required(true)
            .index(1))
        .get_matches();

    match matches.value_of("INPUT") {
        Some(file) => parser::parse(file),
        _ => unreachable!()
    }
}
