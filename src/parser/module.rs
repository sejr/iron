use crate::parser::{ statement, Rule };

#[derive(Debug)]
pub struct Module {
    statements: Vec<statement::Statement>
}

pub fn parse(module: pest::iterators::Pair<'_, Rule>) -> Module {
    let mut statements: Vec<statement::Statement> = Vec::new();
    for node in module.into_inner() {
        match node.as_rule() {
            Rule::statement => statements.push(statement::parse(node)),
            _ => unreachable!()
        }
    }

    Module { statements }
}