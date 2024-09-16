use crate::base_variables::variable::Variable;
use crate::node::node::{IntNode, OperatorNode};
use crate::{base_variables::base_types::BaseTypes, node::node::ASTNode};


pub fn parse_variable_declaration_or_assignment(exp_stack: &mut Vec<ASTNode>) -> bool {
    let mut var_name: Option<String> = None;
    let mut var_type: Option<BaseTypes> = None;
    let mut assignment_operator: Option<String> = None;
    let mut inside_assignment = false;
    let mut value: BaseTypes = BaseTypes::Null;
    let mut var_value = ASTNode::Int(IntNode { value: 0 });
    let mut first: Option<ASTNode> = Option::None;
    let mut parenthesis: bool = false;


    let mut index = 0;
    while index < exp_stack.len() {
        let node = &exp_stack[index];
        match node {
            ASTNode::Variable(v) => {
                var_name = Some(v.value.clone());
            }
            ASTNode::VariableType(v) => {
                var_type = match v.value.as_str() {
                    "int" => Some(BaseTypes::Int(0)),
                    "float" => Some(BaseTypes::Float(0.0)),
                    "string" => Some(BaseTypes::StringWrapper(String::new())),
                    "bool" => Some(BaseTypes::Bool(false)),
                    "char" => Some(BaseTypes::Char('\0')),
                    _ => {
                        println!("Syntax Error: Unrecognized type '{}'", v.value);
                        return false;
                    }
                };
            }
            ASTNode::AssignmentOperator(a) => {
                assignment_operator = Some(a.operator.clone());
                inside_assignment = true;
            }
            ASTNode::Int(n) => {
                if inside_assignment {
                    first = Some(ASTNode::Int(n.clone()));
                    var_value = operation(exp_stack);
                    if let ASTNode::Int(n) = var_value {
                        value = BaseTypes::Int(n.value);
                    } else {
                        println!("Syntax Error: Expected an integer after the operator.");
                        return false;
                    }
                    break;
                }
            }
            ASTNode::LeftParenthesis => {
                parenthesis = true;
            }
            ASTNode::RightParenthesis => {}
            _ => {
                println!("Syntax Error: Unhandled node while parsing variable: {:?}", node);
                return false;
            }
        }
        index += 1;
    }

    if var_name.is_none() || var_type.is_none() || assignment_operator.is_none() {
        println!("Syntax Error: Missing variable components.");
        return false;
    }

    let variable = Variable::new(var_name.unwrap(), value, var_type.unwrap());
    true
}

pub fn operation(exp_stack: &mut Vec<ASTNode>) -> ASTNode {
    
    //cloned_exp_stack.reverse();
    //println!("Cloned stack after reversed: {:?}", cloned_exp_stack);

    //pop 2 elements from the stack
    let mut first_node: Option<ASTNode> = None;
    let mut parenthesis: bool = false;

    exp_stack.reverse();
    exp_stack.pop();
    exp_stack.pop();
    exp_stack.pop(); // Remove the '=' from original stack
    exp_stack.reverse();

    let mut first: bool = false;
    let mut operator: ASTNode = ASTNode::Operator(OperatorNode { operator: String::new() });
    let mut left: ASTNode = ASTNode::Int(IntNode { value: 0 });
    let right: ASTNode = ASTNode::Int(IntNode { value: 0 });

    while let Some(top) = exp_stack.pop() {
        match top {
            ASTNode::Operator(o) => {
                operator = ASTNode::Operator(o);
            }
            ASTNode::Int(n) => {
                if first == false {
                    first = true;
                    left = ASTNode::Int(n);
                } else {
                    let right = ASTNode::Int(n);
                    first_node = Some(parse_operator(&left, &operator, &right));
                }
            }
            ASTNode::RightParenthesis => {
                if parenthesis {
                    break;
                }
            }
            ASTNode::LeftParenthesis => {
                parenthesis = true;
            }
            _ => {
                println!("Syntax Error: Unhandled node in operation: {:?}", top);
                return ASTNode::Int(IntNode { value: 0 });
            }
        }
    }

    if first_node.is_none() {
        return left;
    }
    else {
        return first_node.unwrap();
    }
}

pub fn parse_operator(left: &ASTNode, operator: &ASTNode, right: &ASTNode) -> ASTNode {
    match operator {
        ASTNode::Operator(o) => {
            match o.operator.as_str() {
                "+" => {
                    if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                        let result = left_val.value + right_val.value;
                        let result = IntNode { value: result };
                        return ASTNode::Int(result);
                    }
                }
                "-" => {
                    if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                        let result = left_val.value - right_val.value;
                        let result = IntNode { value: result };
                        return ASTNode::Int(result);
                    }
                }
                "*" => {
                    if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                        let result = left_val.value * right_val.value;
                        let result = IntNode { value: result };
                        return ASTNode::Int(result);
                    }
                }
                "/" => {
                    if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                        if right_val.value != 0 {
                            let result = left_val.value / right_val.value;
                            let result = IntNode { value: result };
                            return ASTNode::Int(result);
                        } else {
                            println!("Syntax Error: Division by zero.");
                            std::process::exit(1);
                        }
                    }
                }
                _ => {
                    println!("Syntax Error: Unrecognized operator '{}'", o.operator);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            println!("Syntax Error: Expected an operator.");
            std::process::exit(1);
        }
    }
    ASTNode::None
}

fn parse_numeric_expression(exp_stack: &mut Vec<ASTNode>) -> BaseTypes {
    let mut evaluate_empty = true;
    let mut evaluate: (ASTNode, ASTNode, ASTNode) = (ASTNode::None, ASTNode::None, ASTNode::None);
    let mut operator = String::new();  // Use a String to store the operator instead of a reference
    let mut first_iter: bool = true;
    let mut result: i32 = 0;

    while let Some(node) = exp_stack.pop() {
        if first_iter {
            first_iter = false;
            continue;
        } else {
            match node {
                ASTNode::Operator(ref o) => {  // Borrow the operator to avoid moving `o`
                    operator = o.operator.clone();  // Clone the operator into the longer-living `String`
                    evaluate.1 = node.clone();  // Clone `node` to avoid partial move issues
                }
                ASTNode::Int(ref n) => {  // Borrow `n` to avoid moving it
                    if evaluate_empty {
                        evaluate.0 = node.clone();  // Clone the node to avoid moving
                        evaluate_empty = false;
                    } else {
                        evaluate.2 = node.clone();  // Clone the node to avoid moving
                    }
                }
                ASTNode::SemiColon => {
                    match operator.as_str() {
                        "+" => {
                            result = match &evaluate.0 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            } + match &evaluate.2 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            };
                        }
                        "-" => {
                            result = match &evaluate.0 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            } - match &evaluate.2 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            };
                        }
                        "*" => {
                            result = match &evaluate.0 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            } * match &evaluate.2 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            };
                        }
                        "/" => {
                            result = match &evaluate.0 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            } / match &evaluate.2 {
                                ASTNode::Int(n) => n.value as i32
,
                                _ => 0,
                            };
                        }
                        _ => {
                            println!("Syntax Error: Unrecognized operator '{}'", operator);
                            std::process::exit(1);
                        }
                    }
                    return BaseTypes::Int(result);
                }
                _ => {}
            }
        }
    }

    BaseTypes::Null
}


fn process_value_expression(exp_stack: &mut Vec<ASTNode>) -> BaseTypes {
    let mut char_buffer = String::new();

    while let Some(node) = exp_stack.pop() {
        match node {
            ASTNode::Int(n) => {
                return BaseTypes::Int(n.value as i32);
            }
            ASTNode::Float(f) => {
                return BaseTypes::Float(f.value.into());
            }
            ASTNode::String(s) => {
                return BaseTypes::StringWrapper(s.value.clone());
            }
            ASTNode::Bool(b) => {
                return BaseTypes::Bool(b.value);
            }
            ASTNode::Char(c) => {
                char_buffer.push(c.value);

                if char_buffer == "true" {
                    return BaseTypes::Bool(true);
                } else if char_buffer == "false" {
                    return BaseTypes::Bool(false);
                } else {
                    return BaseTypes::Char(c.value);
                }
            }
            _ => {
                println!("Unhandled node in value expression: {:?}", node);
            }
        }
    }

    BaseTypes::Null
}
