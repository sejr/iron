use pest::iterators::Pair;
use crate::parser::{
    Rule,
    statement::Statement,
    string
};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Expression {
    Assignment {
        identifier: String,
        kind: Option<String>,
        value: Box<Expression>
    },
    Stack { root: Vec<StackItem> },
    Return { root: Vec<StackItem> },
    Value { as_string: String },
    Null
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum OpSequenceItem {
    Operator { name: Op },
    Identifier { name: String },
    StringValue { value: String },
    BooleanValue { value: bool },
    NumericValue { value: i32 },
    FunctionCall {
        identifier: String,
        arguments: Vec<FunctionArgument>
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum StackItem {
    Singleton { item: OpSequenceItem },
    NestedStack { root: Vec<StackItem> }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Op {
    EqualTo,
    NotEqualTo,
    Add,
    Subtract,
    Pow,
    Multiply,
    Divide,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor
}

#[derive(Clone, Debug)]
pub struct FunctionArgument {
    label: Option<String>,
    value: Vec<StackItem>
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub label: String,
    pub name: String,
    pub kind: String
}

fn parse_param(param: Pair<Rule>) -> Parameter {
    let mut label = String::new();
    let mut name = String::new();
    let mut kind = String::new();

    for node in param.into_inner() {
        match node.as_rule() {
            Rule::label => label = String::from(node.as_str()),
            Rule::identifier => name = String::from(node.as_str()),
            Rule::kind => kind = String::from(node.as_str()),
            _ => println!("UNCHECKED RULE IN PARSE_PARAM: {:?}", node.as_rule())
        }
    }

    Parameter { label, name, kind }
}

fn parse_params(param_list: Pair<Rule>) -> Option<Vec<Parameter>> {
    let mut params: Vec<Parameter> = Vec::new();

    for node in param_list.into_inner() {
        match node.as_rule() {
            Rule::function_parameter => params.push(parse_param(node)),
            _ => println!("UNCHECKED RULE IN PARSE_PARAMS: {:?}", node.as_rule())
        }
    }

    Some(params)
}

fn parse_returns(return_list: Pair<Rule>) -> Option<Vec<String>> {
    let mut returns: Vec<String> = Vec::new();

    for node in return_list.into_inner() {
        match node.as_rule() {
            Rule::kind => returns.push(String::from(node.as_str())),
            _ => println!("UNCHECKED RULE IN PARSE_RETURNS: {:?}", node.as_rule())
        }
    }

    Some(returns)
}

pub fn parse_expression_list(expr: Pair<Rule>) -> Vec<Expression> {
    let mut expressions: Vec<Expression> = Vec::new();
    for node in expr.into_inner() {
        match node.as_rule() {
            Rule::assignment => expressions.push(parse_assignment(node)),
            Rule::return_stmt => expressions.push(parse_return_stmt(node)),
            Rule::op_sequence => {
                let root: Vec<StackItem> = parse_op_sequence(node);
                expressions.push(Expression::Stack { root });
            },
            _ => panic!("Unhandled expression: {:?}", node.as_rule())
        }
    }

    expressions
}

pub fn parse_return_stmt(expr: Pair<Rule>) -> Expression {
    let mut root: Vec<StackItem> = Vec::new();
    for node in expr.into_inner() {
        match node.as_rule() {
            Rule::op_sequence => root = parse_op_sequence(node),
            _ => unreachable!()
        }
    }

    Expression::Return { root }
}

pub fn parse_op_sequence(expr: Pair<Rule>) -> Vec<StackItem> {
    let mut stack: Vec<StackItem> = Vec::new();

    for node in expr.into_inner() {
        match node.as_rule() {
            Rule::string_literal => {
                let item = OpSequenceItem::StringValue {
                    value: string::parse(node)
                };
                let stack_item = StackItem::Singleton { item };
                stack.push(stack_item);
            },
            Rule::boolean_value => {
                let value: bool;
                match node.as_str() {
                    "true" => value = true,
                    "false" => value = false,
                    _ => unreachable!()
                }
                let item = OpSequenceItem::BooleanValue { value };
                let stack_item = StackItem::Singleton { item };
                stack.push(stack_item);
            },
            Rule::identifier => {
                let name = String::from(node.as_str());
                let item = OpSequenceItem::Identifier { name };
                let stack_item = StackItem::Singleton { item };
                stack.push(stack_item);
            }
            Rule::function_call => {
                let item: OpSequenceItem = parse_function_call(node);
                let stack_item = StackItem::Singleton { item };
                stack.push(stack_item);
            },
            Rule::op_sequence => {
                let root: Vec<StackItem> = parse_op_sequence(node);
                let stack_item = StackItem::NestedStack { root };
                stack.push(stack_item);
            },
            Rule::op_eq => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::EqualTo }
                });
            },
            Rule::op_neq => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::NotEqualTo }
                });
            },
            Rule::op_add => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::Add }
                });
            },
            Rule::op_subtract => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::Subtract }
                });
            },
            Rule::op_pow => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::Pow }
                });
            },
            Rule::op_multiply => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::Multiply }
                });
            },
            Rule::op_divide => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::Divide }
                });
            },
            Rule::op_gt => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::GreaterThan }
                });
            },
            Rule::op_gte => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::GreaterThanEqual }
                });
            },
            Rule::op_lt => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::LessThan }
                });
            },
            Rule::op_lte => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::LessThanEqual }
                });
            },
            Rule::logic_and => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::LogicalAnd }
                });
            },
            Rule::logic_or => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::LogicalOr }
                });
            },
            Rule::logic_xor => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::LogicalXor }
                });
            },
            Rule::logic_not => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::LogicalNot }
                });
            },
            Rule::bitwise_and => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::BitwiseAnd }
                });
            },
            Rule::bitwise_or => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::BitwiseOr }
                });
            },
            Rule::bitwise_xor => {
                stack.push(StackItem::Singleton {
                    item: OpSequenceItem::Operator { name: Op::BitwiseXor }
                });
            },
            _ => println!("Unexpected value in sequence: {:?}", node.as_rule())
        }
    }

    stack
}

fn parse_fn_arg(arg: Pair<Rule>) -> FunctionArgument {
    let mut label = None;
    let mut value: Vec<StackItem> = Vec::new();

    for node in arg.into_inner() {
        match node.as_rule() {
            Rule::label => label = Some(String::from(node.as_str())),
            Rule::op_sequence => value = parse_op_sequence(node),
            _ => println!("UNCHECKED RULE: {:?}", node.as_rule())
        }
    }

    FunctionArgument { label, value }
}

fn parse_function_call(fn_call: Pair<Rule>) -> OpSequenceItem {
    let mut identifier = String::new();
    let mut arguments: Vec<FunctionArgument> = Vec::new();

    for node in fn_call.into_inner() {
        match node.as_rule() {
            Rule::identifier => identifier = String::from(node.as_str()),
            Rule::function_arg => arguments.push(parse_fn_arg(node)),
            _ => println!("UNCHECKED RULE: {:?}", node.as_rule())
        }
    }

    OpSequenceItem::FunctionCall { identifier, arguments }
}

pub fn parse_assignment(expr: Pair<Rule>) -> Expression {
    let mut identifier = String::new();
    let mut value = Box::new(Expression::Null);
    let mut kind = None;

    for node in expr.into_inner() {
        match node.as_rule() {
            Rule::identifier => identifier = String::from(node.as_str()),
            Rule::kind => kind = Some(String::from(node.as_str())),
            Rule::op_sequence => {
                let root: Vec<StackItem> = parse_op_sequence(node);
                value = Box::new(Expression::Stack { root });
            },
            _ => println!("{:?}", node.as_rule())
        }
    }

    Expression::Assignment { identifier, value, kind }
}

pub fn parse(function: Pair<Rule>) -> Statement {
    let mut name = String::from("");
    let mut parameters = None;
    let mut returns = None;
    let mut body: Vec<Expression> = Vec::new();
    let mut public = false;

    for node in function.into_inner() {
        match node.as_rule() {
            Rule::public => public = true,
            Rule::identifier => name = String::from(node.as_str()),
            Rule::function_parameter_list => parameters = parse_params(node),
            Rule::returns => returns = parse_returns(node),
            Rule::expression_list => body = parse_expression_list(node),
            _ => println!("UNCHECKED RULE IN PARSE: {:?}", node.as_rule())
        }
    }

    Statement::Function { public, name, parameters, returns, body }
}