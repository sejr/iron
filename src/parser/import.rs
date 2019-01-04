use std::path::Path;
use pest::iterators::Pair;
use crate::parser::{
    Rule,
    string,
    statement::Statement
};

pub fn parse(import: Pair<Rule>) -> Statement {
    let mut path: String = String::new();
    let mut name: String = String::new();
    for node in import.clone().into_inner() {
        match node.as_rule() {
            Rule::string_literal => path = string::parse(node),
            Rule::identifier => name = String::from(node.as_str()),
            _ => print!("")
        }
    }

    // If no identifier was specified by the developer, we need to use the
    // file name as the module name (e.g. "types/Apple.fe" becomes Apple)
    if name.is_empty() {
        let name_str = Path::new(path.as_str())
            .file_stem().expect("Error getting module name")
            .to_str().expect("Error parsing module name");
        name = String::from(name_str);
    }

    Statement::Import { path, name }
}