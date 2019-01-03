use pest::iterators::Pair;
use crate::parser::{ import, function, expression, custom_type, Rule };

#[derive(Debug)]
pub enum Statement {
    Import {
        path: String,
        identifier: Option<String>
    },
    Type {
        public: bool,
        name: String,
        attributes: Vec<(String, String)>
    },
    Function {
        public: bool,
        name: String,
        parameters: Option<Vec<function::Parameter>>,
        returns: Option<Vec<String>>,
        body: Vec<expression::Expression>
    },
    Error
}

pub fn parse(statement: Pair<Rule>) -> Statement {
    let mut s: Statement = Statement::Error;
    let mut _public: bool = false;
    for node in statement.into_inner() {
        match node.as_rule() {
            Rule::import => { s = import::parse(node)},
            Rule::function => { s = function::parse(node)},
            Rule::custom_type => { s = custom_type::parse(node)},
            _ => unreachable!()
        }
    }

    return s;
}