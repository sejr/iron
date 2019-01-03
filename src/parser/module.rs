use pest::iterators::Pair;
use crate::parser::{ statement, Rule };
use crate::parser::statement::Statement;

#[derive(Debug)]
pub struct Module {
    name: String,
    statements: Vec<statement::Statement>
}

pub fn parse(name: &str, module: Pair<Rule>) -> Module {
    let mut statements: Vec<statement::Statement> = Vec::new();
    for node in module.into_inner() {
        match node.as_rule() {
            Rule::statement => {
                let s: Statement = statement::parse(node);
                statements.push(s);
            },
            _ => unreachable!()
        }
    }

    Module {
        name: String::from(name),
        statements
    }
}