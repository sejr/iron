use std::fs;
use pest::Parser;
use std::path::Path;

pub mod module;
pub mod statement;
pub mod function;
pub mod import;
pub mod string;
pub mod expression;
pub mod custom_type;

#[derive(Parser)]
#[grammar = "../grammar/iron.pest"]
pub struct IronParser;

pub fn parse(path: &str) {
    let module_name = Path::new(path)
        .file_stem()
        .expect("File not found")
        .to_str().expect("Error getting file name");

    let data = fs::read_to_string(path).expect("Unable to read file");
    let ast = IronParser::parse(Rule::module, data.as_str())
        .unwrap_or_else(|e| panic!("{}", e));

    for node in ast {
        match node.as_rule() {
            Rule::module => println!("{:?}", module::parse(module_name, node)),
            _ => unreachable!()
        }
    }
}