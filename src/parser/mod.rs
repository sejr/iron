use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "../grammar/iron.pest"]
pub struct IronParser;

pub fn parse(path: &str) {
    let data = fs::read_to_string(path).expect("Unable to read file");
    let ast = IronParser::parse(Rule::module, data.as_str())
        .unwrap_or_else(|e| panic!("{}", e));

    for node in ast {
        match node.as_rule() {
            Rule::module => parse_module(node),
            _ => unreachable!()
        }
    }
}

fn parse_module(module: pest::iterators::Pair<'_, Rule>) {
    for node in module.into_inner() {
        match node.as_rule() {
            Rule::statement => parse_statement(node),
            _ => unreachable!()
        }
    }
}

fn parse_statement(statement: pest::iterators::Pair<'_, Rule>) {
    for node in statement.into_inner() {
        match node.as_rule() {
            Rule::import => parse_import(node),
            Rule::function => parse_function(node),
            _ => unreachable!()
        }
    }
}

fn parse_import(import: pest::iterators::Pair<'_, Rule>) {
    println!("Parsed import\n{:?}\n", import.as_str());
}

fn parse_function(function: pest::iterators::Pair<'_, Rule>) {
    println!("Parsed function\n{:?}\n", function.as_str());
}