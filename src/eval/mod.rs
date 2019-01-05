use crate::parser::module::Module;

#[allow(dead_code)]
pub fn eval(module: Module) {
    println!("Evaluating module: {:?}", module.name);
}