use crate::parser::{
    Rule,
    string,
    statement::Statement
};

pub fn parse(import: pest::iterators::Pair<'_, Rule>) -> Statement {
    let mut path = String::new();
    for node in import.clone().into_inner() {
        match node.as_rule() {
            Rule::string_literal => path = string::parse(node),
            _ => print!("")
        }
    }

    Statement::Import { path }
}