use crate::parser::{ import, function, custom_type, Rule };

#[derive(Debug)]
pub enum Statement {
    Import {
        path: String,
        identifier: Option<String>
    },
    Type {
        name: String,
        attributes: Vec<(String, String)>,
        public: bool
    },
    Function,
    Error
}

pub fn parse(statement: pest::iterators::Pair<'_, Rule>) -> Statement {
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