use crate::parser::{ import, function, Rule };

#[derive(Debug)]
pub enum Statement {
    Import,
    Function,
    Error
}

pub fn parse(statement: pest::iterators::Pair<'_, Rule>) -> Statement {
    let mut s: Statement = Statement::Error;
    for node in statement.into_inner() {
        match node.as_rule() {
            Rule::import => { s = import::parse(node)},
            Rule::function => { s = function::parse(node)},
            _ => unreachable!()
        }
    }

    return s;
}