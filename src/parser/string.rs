use crate::parser::{ Rule };

pub fn parse(import: pest::iterators::Pair<'_, Rule>) -> String {
    let mut string = String::new();
    for node in import.clone().into_inner() {
        match node.as_rule() {
            Rule::string => string = String::from(node.as_str()),
            _ => unreachable!()
        }
    }
    
    string
}