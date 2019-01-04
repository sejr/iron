use colored::*;
use std::time::{ Instant, Duration };
use crate::parser::{
    module::Module,
    statement::Statement
};

pub fn print_statement(stmt: &Statement) {
    match stmt {
        Statement::Import { path, name } => {
            println!("{}: \"{}\"", name.yellow(), path);
        },
        Statement::Function { public, name, parameters, returns, .. } => {
            if *public {
                print!("{}", "public  ".green());
            } else {
                print!("        ");
            }

            print!("{}", name.yellow());
            if let Some(parameters) = parameters {
                print!(" (");
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
                    print!(" -> {}", ret.cyan().italic());
                }
            }

            print!("\n");
        },
        _ => println!("{:?}", stmt)
    }
}

pub fn profile(parse_time: Duration, module: Module) {
    println!("{} Module parsed in {:?}", "[PROFILER]".cyan(), parse_time);
    let profile_begin = Instant::now();
    module.print_profile();
    let profile_end = Instant::now();
    println!("{} Profiled in {:?}", "[PROFILER]".cyan(),
        profile_end.duration_since(profile_begin));
}
