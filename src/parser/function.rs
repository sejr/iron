use crate::parser::{ Rule, statement::Statement };

pub fn parse(function: pest::iterators::Pair<'_, Rule>) -> Statement {
    println!("Parsed function\n{:?}\n", function.as_str());
    Statement::Function
}