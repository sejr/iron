use crate::parser::{ Rule, statement::Statement };

pub fn parse(_function: pest::iterators::Pair<'_, Rule>) -> Statement {
    Statement::Function
}