use colored::*;
use pest::iterators::Pair;
use std::collections::HashMap;

use crate::util::print_statement;
use crate::parser::{ statement, Rule };
use crate::parser::statement::Statement;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub environment: HashMap<String, Statement>
}

#[allow(dead_code)]
impl Module {
    pub fn print_profile(&self) {
        println!();
        println!("{}:\t{}", "MODULE".yellow().bold(), self.name);

        let entry = self.entry();
        if let Some(_entry) = entry {
            println!("{}:\tmain", "ENTRY".yellow().bold());
        }

        let imports = self.imports();
        if let Some(imports) = imports {
            if imports.len() > 0 {
                println!("{}", "IMPORTS".yellow().bold());
                for import in imports {
                    print_statement(import);
                }
            }
        }

        let functions = self.functions();
        if let Some(functions) = functions {
            if functions.len() > 0 {
                println!("{}:", "FUNCTIONS".yellow().bold());
                for function in functions {
                    print_statement(function);
                }
            }
        }
        
        println!();
    }

    pub fn entry(&self) -> Option<&Statement> {
        self.environment.get("main")
    }

    pub fn imports(&self) -> Option<Vec<&Statement>> {
        self.environment.keys().filter(|statement| {
            match self.environment
                .get(*statement)
                .expect("Something went wrong") {

                Statement::Import { .. } => return true,
                _ => return false
            }
        }).map(|key| {
            return self.environment.get(key);
        }).collect()
    }

    // Note that this function does not return the `main` function. This is
    // because `main` serves as the entry point to the module and is never meant
    // to be accessed outside the module.
    pub fn functions(&self) -> Option<Vec<&Statement>> {
        self.environment.keys().filter(|statement| {
            match self.environment
                .get(*statement)
                .expect("Something went wrong") {

                Statement::Function { name, .. } => {
                    return *name != String::from("main");
                },
                _ => return false
            }
        }).map(|key| {
            return self.environment.get(key);
        }).collect()
    }

    pub fn types(&self) -> Option<Vec<&Statement>> {
        self.environment.keys().filter(|statement| {
            match self.environment
                .get(*statement)
                .expect("Something went wrong") {

                Statement::Type { .. } => return true,
                _ => return false
            }
        }).map(|key| {
            return self.environment.get(key);
        }).collect()
    }
}

pub fn parse(name: &str, module: Pair<Rule>) -> Module {
    let mut environment: HashMap<String, Statement> = HashMap::new();
    for node in module.into_inner() {
        match node.as_rule() {
            Rule::statement => {
                let s: Statement = statement::parse(node);
                match s.clone() {
                    Statement::Import { name, .. } => {
                        environment.insert(name, s);
                    },
                    Statement::Function { name, .. } => {
                        environment.insert(name, s);
                    },
                    Statement::Type { name, .. } => {
                        environment.insert(name, s);
                    }
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }

    Module {
        name: String::from(name),
        environment
    }
}