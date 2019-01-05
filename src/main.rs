extern crate clap;
extern crate pest;
extern crate colored;

#[macro_use]
extern crate pest_derive;

mod util;
mod parser;

use std::fs::File;
use std::path::Path;
use clap::{ App, Arg };
use std::time::Instant;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let matches = App::new("Forge").version("0.0.1")
        .author("Samuel Roth <sam@roth.fyi>")
        .about("Iron language toolkit")

        // If this flag is passed, we will display information about the
        // module rather than evaluate it. This provides a quick glance of
        // the available functions, types, etc. I will probably modify this
        // to include doc comments and support for external modules.
        .arg(Arg::with_name("info")
            .multiple(true)
            .help("Display information for provided module")
            .short("i")
            .long("info"))
        
        // This is the Iron module that we want to evaluate or analyze.
        .arg(Arg::with_name("module")
            .help("Iron module to evaluate")
            .required(true))
        .get_matches();

    // Because we define the module parameter as required in our Clap app, 
    // we do not need to define the `else` branch. If the user does not provide
    // a file path as input, Clap will display its own error.
    let file = matches.value_of("module");
    if let Some(file) = file {

        // We want to keep track of the amount of time we spend parsing this
        // module. This information can be displayed by passing the `-i` flag.
        let parsing_start = Instant::now();
        if let Some(module) = parser::parse(file) {

            // How long did it take for this module to be parsed?
            let parse_time = Instant::now().duration_since(parsing_start);

            // We have two options for displaying information about the target
            // module. The first option is to display our nicely-formatted
            // listing of all the interfaces. The second option, meant more for
            // development purposes, displays the raw Rust `module` struct.
            //
            // If there are zero occurrences of the `-i` flag, then we just
            // evaluate the module as expected.
            match matches.occurrences_of("info") {
                0 => Ok(()),
                1 => Ok(util::profile(parse_time, module)),
                _ => {
                    let mod_str = format!("{}.ast", file);
                    let path = Path::new(mod_str.as_str());
                    let mut file = File::create(path)?;
                    let module_debug = format!("{:#?}", module);
                    file.write_all(module_debug.as_bytes())?;
                    file.sync_all()?;
                    Ok(())
                },
            }
        
        // Something went while parsing the provided module. This error should
        // not appear because all parsing errors should be handled by Pest or
        // the Iron parser itself.
        } else {
            panic!("Error parsing module.");
        }
    } else {
        panic!("Something went wrong");
    }
}
