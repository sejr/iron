use colored::*;
use std::time::{ Instant, Duration };
use crate::parser::{
    module::Module,
    statement::Statement
};

// This is a helper function that takes a statement object and prints it in a
// human-readable way. We specifically try to limit the length of the statement
// to 80 characters so the entire statement is easily viewable in a terminal
// environment.
pub fn print_statement(stmt: &Statement) {
    match stmt {

        // For an import, we just need its name and the path from which it
        // was retrieved.
        Statement::Import { path, name } => {
            println!("{}: \"{}\"", name.yellow(), path);
        },

        // For a function, we need its name, whether or not it is public, as
        // well as its parameter and return types, if any.
        Statement::Function { public, name, parameters, returns, .. } => {
            if *public {
                print!("{}", "public  ".green());
            } else {
                print!("        ");
            }

            print!("{}", name.yellow());
            if let Some(parameters) = parameters {
                print!("(");
                for parameter in parameters {
                    print!("{} {}: {}", 
                        parameter.label,
                        parameter.name.white(), 
                        parameter.kind.cyan().italic());
                }
                print!(")");
            }

            if let Some(returns) = returns {
                for ret in returns {
                    print!(": {}", ret.cyan().italic());
                }
            }

            // We only want to show one function on each line.
            print!("\n");
        },

        // We should not reach this point, but if we do, we just print the
        // raw statement object. This is mostly for debugging purposes on
        // the development side.
        _ => println!("{:?}", stmt)
    }
}

// This function does the heavy lifting of interacting with the profiler via
// the Forge CLI. This also prints some timing information that we recorded
// when parsing the Iron file.
pub fn profile(parse_time: Duration, module: Module) {
    // Display module parsing time
    println!("{} Module parsed in {:?}", "[PROFILER]".cyan(), parse_time);

    // We record the time it takes to generate the profile because it offers
    // insight into how long it takes to retrieve entities from the module
    // environment, such as imports, functions, entry point, etc.
    let profile_begin = Instant::now();
    module.print_profile();

    // Finally, we print the time it took to generate the profile.
    println!("{} Profiled in {:?}", "[PROFILER]".cyan(),
        Instant::now().duration_since(profile_begin));
}
