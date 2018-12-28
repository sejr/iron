use crate::parser::{ Rule, statement::Statement };

pub fn parse(import: pest::iterators::Pair<'_, Rule>) -> Statement {
    println!("Parsed import\n{:?}\n", import.as_str());
    Statement::Import
}