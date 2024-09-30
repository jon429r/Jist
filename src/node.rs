/*
* This file contains the different AST node types used to create the AST tree and functions to go
* with them
*/

pub mod nodes {
    use crate::base_variable::base_types::BaseTypes;
    use crate::token_type::token_types::*;
    use crate::tokenizer::tokenizers::ParseInfo;
    use std::fmt;
    use std::fmt::Write;

    pub fn to_base_type(node: &ASTNode) -> Option<BaseTypes> {
        match node {
            ASTNode::SemiColon => Some(BaseTypes::Null),
            ASTNode::Operator(_) => Some(BaseTypes::Null),
            ASTNode::Int(int_node) => Some(BaseTypes::Int(int_node.value)),
            ASTNode::String(string_node) => {
                Some(BaseTypes::StringWrapper(string_node.value.clone()))
            }
            ASTNode::Char(char_node) => Some(BaseTypes::Char(char_node.value)),
            ASTNode::Bool(bool_node) => Some(BaseTypes::Bool(bool_node.value)),
            ASTNode::Float(float_node) => Some(BaseTypes::Float(float_node.value as f64)),
            ASTNode::Assignment(_) => Some(BaseTypes::Null),
            ASTNode::Collection(_) => Some(BaseTypes::Null),
            ASTNode::VarTypeAssignment(_) => Some(BaseTypes::Null),
            ASTNode::Variable(_) => Some(BaseTypes::Null),
            ASTNode::Function(_) => Some(BaseTypes::Null),
            ASTNode::FunctionCall(_) => Some(BaseTypes::Null),
            ASTNode::VariableCall(_) => Some(BaseTypes::Null),
            ASTNode::VariableType(_) => Some(BaseTypes::Null),
            ASTNode::VariableValue(_) => Some(BaseTypes::Null),
            ASTNode::FunctionArguments(_) => Some(BaseTypes::Null),
            ASTNode::AssignmentOperator(_) => Some(BaseTypes::Null),
            ASTNode::ReturnTypeAssignment(_) => Some(BaseTypes::Null),
            ASTNode::Comment(_) => Some(BaseTypes::Null),
            ASTNode::FunctionCallArguments(_) => Some(BaseTypes::Null),
            ASTNode::LeftParenthesis => Some(BaseTypes::Null),
            ASTNode::RightParenthesis => Some(BaseTypes::Null),
            ASTNode::ArgumentSeparator => Some(BaseTypes::Null),
            ASTNode::LeftCurly => Some(BaseTypes::Null),
            ASTNode::RightCurly => Some(BaseTypes::Null),
            ASTNode::RightBracket => Some(BaseTypes::Null),
            ASTNode::LeftBracket => Some(BaseTypes::Null),
            ASTNode::None => Some(BaseTypes::Null),
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ASTNode {
        SemiColon,
        Operator(OperatorNode),
        Int(IntNode),
        String(StringNode),
        Char(CharNode),
        Bool(BoolNode),
        Float(FloatNode),
        Assignment(AssignmentNode),
        VarTypeAssignment(VarTypeAssignmentNode),
        Variable(VariableNode),
        Function(FunctionNode),
        FunctionCall(FunctionCallNode),
        VariableCall(VariableCallNode),
        VariableType(VariableTypeNode),
        VariableValue(VariableValueNode),
        FunctionArguments(FunctionArgumentsNode),
        AssignmentOperator(AssignmentOperatorNode),
        ReturnTypeAssignment(ReturnTypeAssignmentNode),
        Comment(CommentNode),
        FunctionCallArguments(FunctionArgumentsNode),
        Collection(CollectionNode),
        LeftBracket,
        RightBracket,
        LeftParenthesis,
        RightParenthesis,
        ArgumentSeparator,
        LeftCurly,
        RightCurly,
        None,
    }

    impl fmt::Display for ASTNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ASTNode::Collection(c) => write!(f, "{}", c),
                ASTNode::SemiColon => write!(f, "SemiColon"),
                ASTNode::Operator(o) => write!(f, "{}", o),
                ASTNode::Int(i) => write!(f, "{}", i),
                ASTNode::String(s) => write!(f, "{}", s),
                ASTNode::Char(c) => write!(f, "{}", c),
                ASTNode::Bool(b) => write!(f, "{}", b),
                ASTNode::Float(fl) => write!(f, "{}", fl),
                ASTNode::Assignment(a) => write!(f, "{}", a),
                ASTNode::VarTypeAssignment(v) => write!(f, "{}", v),
                ASTNode::Variable(v) => write!(f, "{}", v),
                ASTNode::Function(fun) => write!(f, "{}", fun),
                ASTNode::FunctionCall(fun) => write!(f, "{}", fun),
                ASTNode::VariableCall(v) => write!(f, "{}", v),
                ASTNode::VariableType(v) => write!(f, "{}", v),
                ASTNode::VariableValue(v) => write!(f, "{}", v),
                ASTNode::AssignmentOperator(a) => write!(f, "{}", a),
                ASTNode::ReturnTypeAssignment(r) => write!(f, "{}", r),
                ASTNode::Comment(c) => write!(f, "{}", c),
                ASTNode::LeftParenthesis => write!(f, "LeftParenthesis"),
                ASTNode::RightParenthesis => write!(f, "RightParenthesis"),
                ASTNode::ArgumentSeparator => write!(f, "ArgumentSeparator"),
                ASTNode::LeftCurly => write!(f, "LeftCurly"),
                ASTNode::RightCurly => write!(f, "RightCurly"),
                ASTNode::RightBracket => write!(f, "RightBracket"),
                ASTNode::LeftBracket => write!(f, "LeftBracket"),
                ASTNode::FunctionCallArguments(call_args) => write!(f, "{}", call_args), // Call Display
                ASTNode::FunctionArguments(args) => write!(f, "{}", args), // Call Display
                ASTNode::None => write!(f, "None"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CollectionNode {
        pub name: String,
    }

    impl CollectionNode {
        pub fn new(name: String) -> Self {
            CollectionNode { name }
        }
        pub fn display_info(&self) {
            println!("Collection: {}", self.name);
        }
    }

    impl fmt::Display for CollectionNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Collection: {}", self.name)
        }
    }

    // BoolNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct BoolNode {
        pub value: bool,
    }

    impl BoolNode {
        pub fn new(value: bool) -> Self {
            BoolNode { value }
        }
        pub fn display_info(&self) {
            println!("Bool: {}", self.value);
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FloatNode {
        pub value: f32,
    }

    impl FloatNode {
        pub fn new(value: f32) -> Self {
            FloatNode { value }
        }
        pub fn display_info(&self) {
            println!("Float: {}", self.value);
        }
    }

    impl fmt::Display for FloatNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Float: {}", self.value)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FunctionArgumentsNode {
        pub value: String,
    }

    impl FunctionArgumentsNode {
        pub fn new(value: String) -> Self {
            FunctionArgumentsNode { value }
        }

        pub fn display_info(&self) {
            println!("Function Arguments: {}", self.value);
        }
    }

    impl fmt::Display for FunctionArgumentsNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Function Arguments: {}", self.value)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ArgumentCallNode {
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FunctionCallArgumentsNode {
        pub value: String,
    }

    impl FunctionCallArgumentsNode {
        pub fn new(value: String) -> Self {
            FunctionCallArgumentsNode { value }
        }
        pub fn display_info(&self) {
            println!("Function Call Arguments: {}", self.value);
        }
    }

    impl fmt::Display for FunctionCallArgumentsNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Function Call Arguments: {}", self.value)
        }
    }

    // CommentNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct CommentNode {
        pub value: String,
    }

    impl CommentNode {
        pub fn new(value: String) -> Self {
            CommentNode { value }
        }
        pub fn display_info(&self) {
            println!("Comment: {}", self.value);
        }
    }

    impl fmt::Display for CommentNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Comment: {}", self.value)
        }
    }

    impl fmt::Display for BoolNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Bool: {}", self.value)
        }
    }

    // SemiColonNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct SemiColonNode;

    impl SemiColonNode {
        pub fn new() -> Self {
            SemiColonNode
        }
        pub fn display_info(&self) {
            println!("SemiColon");
        }
    }

    impl fmt::Display for SemiColonNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "SemiColon")
        }
    }

    // AssignmentOperatorNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct AssignmentOperatorNode {
        pub operator: String,
    }

    impl AssignmentOperatorNode {
        pub fn new(operator: String) -> Self {
            AssignmentOperatorNode { operator }
        }
        pub fn display_info(&self) {
            println!("Assignment Operator: {}", self.operator);
        }
    }

    impl fmt::Display for AssignmentOperatorNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Assignment Operator: {}", self.operator)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ReturnTypeAssignmentNode {
        pub value: String,
    }

    impl ReturnTypeAssignmentNode {
        pub fn new(value: String) -> Self {
            ReturnTypeAssignmentNode { value }
        }
        pub fn display_info(&self) {
            println!("Return Type Assignment: {}", self.value);
        }
    }

    impl fmt::Display for ReturnTypeAssignmentNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Return Type Assignment: {}", self.value)
        }
    }

    // OperatorNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct OperatorNode {
        pub operator: String,
    }

    impl OperatorNode {
        pub fn new(operator: String) -> Self {
            OperatorNode { operator }
        }
        pub fn display_info(&self) {
            println!("Operator: {}", self.operator);
        }
    }

    impl fmt::Display for OperatorNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Operator: {}", self.operator)
        }
    }

    // IntNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct IntNode {
        pub value: i32,
    }

    impl IntNode {
        pub fn new(value: i32) -> Self {
            IntNode { value }
        }
        pub fn display_info(&self) {
            println!("Int: {}", self.value);
        }
    }

    impl fmt::Display for IntNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Int: {}", self.value)
        }
    }

    // StringNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct StringNode {
        pub value: String,
    }

    impl StringNode {
        pub fn new(value: String) -> Self {
            StringNode { value }
        }
        pub fn display_info(&self) {
            println!("String: {}", self.value);
        }
    }

    impl fmt::Display for StringNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "String: {}", self.value)
        }
    }

    // CharNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct CharNode {
        pub value: char,
    }

    impl CharNode {
        pub fn new(value: char) -> Self {
            CharNode { value }
        }
        pub fn display_info(&self) {
            println!("Char: {}", self.value);
        }
    }

    impl fmt::Display for CharNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Char: {}", self.value)
        }
    }

    // AssignmentNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct AssignmentNode {
        pub value: String,
    }

    impl AssignmentNode {
        pub fn new(value: String) -> Self {
            AssignmentNode { value }
        }
        pub fn display_info(&self) {
            println!("Assignment: {}", self.value);
        }
    }

    impl fmt::Display for AssignmentNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Assignment: {}", self.value)
        }
    }

    // VarTypeAssignmentNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct VarTypeAssignmentNode {
        pub value: String,
    }

    impl VarTypeAssignmentNode {
        pub fn new(value: String) -> Self {
            VarTypeAssignmentNode { value }
        }
        pub fn display_info(&self) {
            println!("VarTypeAssignment: {}", self.value);
        }
    }

    impl fmt::Display for VarTypeAssignmentNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "VarTypeAssignment: {}", self.value)
        }
    }

    // VariableNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct VariableNode {
        pub var_type: String,
        pub value: String,
    }

    impl VariableNode {
        pub fn new(var_type: String, value: String) -> Self {
            VariableNode { var_type, value }
        }
        pub fn display_info(&self) {
            println!("Variable Type: {}, Value: {}", self.var_type, self.value);
        }
    }

    impl fmt::Display for VariableNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Variable Type: {}, Value: {}", self.var_type, self.value)
        }
    }

    // VariableValueNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct VariableValueNode {
        pub value: String,
    }

    impl VariableValueNode {
        pub fn new(value: String) -> Self {
            VariableValueNode { value }
        }
        pub fn display_info(&self) {
            println!("Variable Value: {}", self.value);
        }
    }

    impl fmt::Display for VariableValueNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Variable Value: {}", self.value)
        }
    }

    // VariableTypeNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct VariableTypeNode {
        pub value: String,
    }

    impl VariableTypeNode {
        pub fn new(value: String) -> Self {
            VariableTypeNode { value }
        }
        pub fn display_info(&self) {
            println!("Variable Type: {}", self.value);
        }
    }

    impl fmt::Display for VariableTypeNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Variable Type: {}", self.value)
        }
    }
    // VariableCall    // VariableCallNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct VariableCallNode {
        pub name: String,
    }

    impl VariableCallNode {
        pub fn new(name: String) -> Self {
            VariableCallNode { name }
        }
        pub fn display_info(&self) {
            println!("Variable Call: {}", self.name);
        }
    }

    impl fmt::Display for VariableCallNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Variable Call: {}", self.name)
        }
    }

    // FunctionNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct FunctionNode {
        pub name: String,
    }

    impl FunctionNode {
        pub fn new(name: String) -> Self {
            FunctionNode { name }
        }
        pub fn display_info(&self) {
            println!("Function: {}", self.name);
        }
    }

    impl fmt::Display for FunctionNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Function: {}", self.name)
        }
    }

    // FunctionCallNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct FunctionCallNode {
        pub name: String,
    }

    impl FunctionCallNode {
        pub fn new(name: String) -> Self {
            FunctionCallNode { name }
        }
        pub fn display_info(&self) {
            println!("Function Call: {}", self.name);
        }
    }

    impl fmt::Display for FunctionCallNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Function Call: {}", self.name)
        }
    }

    pub fn match_token_to_node(parse_info: ParseInfo) -> ASTNode {
        match parse_info.token {
            TokenTypes::Int => {
                if let Ok(value) = parse_info.value.parse::<i32>() {
                    ASTNode::Int(IntNode::new(value))
                } else if let Ok(value) = parse_info.value.parse::<i32>() {
                    ASTNode::Int(IntNode::new(value as i32))
                } else {
                    panic!("Failed to parse Int: {}", parse_info.value);
                }
            }
            TokenTypes::String => ASTNode::String(StringNode::new(parse_info.value)),
            TokenTypes::Bool => ASTNode::Bool(BoolNode::new(
                parse_info.value.parse::<bool>().expect("Invalid bool"),
            )),
            TokenTypes::Float => ASTNode::Float(FloatNode::new(
                parse_info.value.parse::<f32>().expect("Invalid float"),
            )),
            TokenTypes::Char => ASTNode::Char(CharNode::new(
                parse_info.value.chars().next().expect("Invalid char"),
            )),
            TokenTypes::Operator => ASTNode::Operator(OperatorNode::new(parse_info.value)),
            TokenTypes::AssignmentOperator => {
                ASTNode::AssignmentOperator(AssignmentOperatorNode::new(parse_info.value))
            }
            TokenTypes::LeftParenthesis => ASTNode::LeftParenthesis,
            TokenTypes::RightParenthesis => ASTNode::RightParenthesis,
            TokenTypes::Function => ASTNode::Function(FunctionNode::new(parse_info.value)),
            TokenTypes::FunctionArguments => {
                ASTNode::FunctionArguments(FunctionArgumentsNode::new(parse_info.value))
            }
            TokenTypes::FunctionCall => {
                ASTNode::FunctionCall(FunctionCallNode::new(parse_info.value))
            }
            TokenTypes::Variable => {
                ASTNode::Variable(VariableNode::new("".to_string(), parse_info.value))
            }
            TokenTypes::VarTypeAssignment => {
                ASTNode::VariableType(VariableTypeNode::new(parse_info.value))
            }
            TokenTypes::VariableCall => {
                ASTNode::VariableCall(VariableCallNode::new(parse_info.value))
            }
            TokenTypes::ArgumentSeparator => ASTNode::ArgumentSeparator,
            TokenTypes::Assignment => ASTNode::Assignment(AssignmentNode::new(parse_info.value)),
            TokenTypes::RightCurly => ASTNode::RightCurly,
            TokenTypes::LeftCurly => ASTNode::LeftCurly,
            TokenTypes::ReturnTypeAssignment => {
                ASTNode::ReturnTypeAssignment(ReturnTypeAssignmentNode::new(parse_info.value))
            }
            TokenTypes::Comment => ASTNode::Comment(CommentNode::new(parse_info.value)),
            TokenTypes::SemiColon => ASTNode::SemiColon,
            TokenTypes::None => ASTNode::None,
            TokenTypes::Collection => ASTNode::Collection(CollectionNode::new(parse_info.value)),
            TokenTypes::LeftBracket => ASTNode::LeftBracket,
            TokenTypes::RightBracket => ASTNode::RightBracket,
            _ => {
                panic!("Unrecognized token: {:?}", parse_info.token);
            }
        }
    }
}
