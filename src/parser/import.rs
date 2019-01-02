use crate::parser::{
    Rule,
    string,
    statement::Statement
};

pub fn parse(import: pest::iterators::Pair<'_, Rule>) -> Statement {
    let mut path: String = String::new();
    let mut identifier: Option<String> = None;
    for node in import.clone().into_inner() {
        match node.as_rule() {
            Rule::string_literal => path = string::parse(node),
            Rule::identifier => identifier = Some(String::from(node.as_str())),
            _ => print!("")
        }
    }

    Statement::Import {
        path,
        identifier
    }
}