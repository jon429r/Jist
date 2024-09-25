mod ast;
pub mod base_variables;
pub mod compiler;
mod function_map;
mod node;
pub mod token_types;
mod tokenizer;

mod compilers {
    pub mod function;
    pub mod operation;
    pub mod variable;
}

use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::exit;

use base_variables::variables::VARIABLE_STACK;
use compiler::compiler::route_to_parser;
use node::node::match_token_to_node;
use node::node::ASTNode;
use tokenizer::tokenizer::tokenize;

fn check_file_extension(file_path: String) -> Result<bool, Box<dyn Error>> {
    let ext = Path::new(&file_path).extension().and_then(OsStr::to_str);

    let valid_ext = "jist"; // Remove the leading dot

    if ext == Some(valid_ext) {
        Ok(true)
    } else {
        Err("Invalid file extension".into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No file path provided".into());
    }
    let file_path = &args[1];

    // Check file extension
    match check_file_extension(file_path.to_owned()) {
        Ok(true) => {
            //println!("File path is valid");
        }
        Err(_) => {
            return Err("File path not valid: Does not have extension .jist".into());
        }
        _ => {
            return Err("Some error occurred".into());
        }
    }

    // Read the file contents
    let contents = fs::read_to_string(file_path)?;
    //println!("{}", contents);

    // Tokenize each line and collect AST nodes
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let _ast_nodes: Vec<ASTNode> = Vec::new();

    for line in lines {
        let tokens = tokenize(line);
        println!("Tokens {:?}", tokens);
        //
        let mut hasroot = false;
        let mut tokenized_expression = Vec::new();
        for parse_info in tokens {
            let node = match_token_to_node(parse_info);
            //println!("Node: {:?}", node);

            match node {
                ASTNode::SemiColon => {
                    if !hasroot {
                        //throw syntax error
                        print!("Syntax error expression must be more than semicolon");
                        exit(1);
                    } else {
                        //send to parser module
                        route_to_parser(&mut tokenized_expression);
                    }
                }
                _ => {
                    if !hasroot {
                        hasroot = true;
                        tokenized_expression.push(node);
                    } else {
                        tokenized_expression.push(node);
                    }
                }
            }
        }

        // Display the collected AST nodes
        /*
        for node in &ast_nodes {
            let indent = " ".repeat(4);
            match node {
                ASTNode::Bool(b) => println!("{}BoolNode: Value: {}", indent, b.value),
                ASTNode::Variable(v) => println!("{}VariableNode: Type: {}, Value: {}", indent, v.var_type, v.value),
                ASTNode::Int(n) => println!("{}IntNode: Value: {}", indent, n.value),
                ASTNode::Operator(o) => println!("{}OperatorNode: Operator: {}", indent, o.operator),
                ASTNode::Function(f) => println!("{}FunctionNode: Name: {}", indent, f.name),
                ASTNode::String(s) => println!("{}StringNode: Value: {}", indent, s.value),
                ASTNode::Char(c) => println!("{}CharNode: Value: {}", indent, c.value),
                ASTNode::Assignment(a) => println!("{}AssignmentNode: Value: {}", indent, a.value),
                ASTNode::VarTypeAssignment(v) => println!("{}VarTypeAssignmentNode: Value: {}", indent, v.value),
                ASTNode::FunctionCall(f) => println!("{}FunctionCallNode: Value: {}", indent, f.name),
                ASTNode::VariableCall(v) => println!("{}VariableCallNode: Value: {}", indent, v.name),
                ASTNode::VariableType(v) => println!("{}VariableTypeNode: Value: {}", indent, v.value),
                ASTNode::VariableValue(v) => println!("{}VariableValueNode: Value: {}", indent, v.value),
                ASTNode::FunctionArguments(f) => println!("{}FunctionArgumentsNode: Value: {}", indent, f.value),
                ASTNode::AssignmentOperator(a) => println!("{}AssignmentOperatorNode: Value: {}", indent, a.operator),
                ASTNode::ReturnTypeAssignment(r) => println!("{}ReturnTypeAssignmentNode: Value: {}", indent, r.value),
                ASTNode::Comment(c) => println!("{}CommentNode: Value: {}", indent, c.value),
                ASTNode::SemiColon => println!("{}SemicolonNode", indent),
                ASTNode::LeftParenthesis => println!("{}LeftParenthesisNode", indent),
                ASTNode::RightParenthesis => println!("{}RightParenthesisNode", indent),
                ASTNode::ArgumentSeparator => println!("{}ArgumentSeparatorNode", indent),
                ASTNode::LeftCurly => println!("{}LeftCurlyNode", indent),
                ASTNode::RightCurly => println!("{}RightCurlyNode", indent),
                ASTNode::None => println!("{}NoneNode", indent),
            }
        }*/
    }
    //print variable stack
    println!("\nVariable stack:");
    for variable in unsafe { VARIABLE_STACK.iter() } {
        variable.print();
    }

    Ok(())
}
