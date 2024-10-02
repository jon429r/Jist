/*
* This file contains the token types used by the tokenizer, more can be added as needed to process
* a variety of syntax expressions
*/

pub mod token_types {

    #[derive(Debug, Clone)]
    pub enum TokenTypes {
        /*
         * A simple numeric value
         */
        Int,
        /*
         * A simple string value
         */
        String,
        /*
         * A simple character value
         */
        Char,
        /*
         * the = operator
         */
        AssignmentOperator,

        /*
         * either true or false
         */
        Bool,
        /*
         * A function argument
         */
        FunctionArguments,
        /*
         * An operator (+,-,/,*)
         */
        Operator,
        /*
         * (
         */
        LeftParenthesis,
        /*
         * )
         */
        RightParenthesis,
        /*
         * func
         */
        Function,
        /*
         * funcname()
         */
        FunctionCall,
        /*
         * ',' Comma used to separate function arguments
         */
        ArgumentSeparator,
        /*
         * a = 2
         */
        VariableCall,
        /*
         * values within () in a function call
         */
        FunctionCallArguments,
        /*
         * 'let' used to declare a variable
         */
        Assignment,
        /*
         * '}'
         */
        RightCurly,
        /*
         * '{'
         */
        LeftCurly,
        /*
         *let
         */
        Variable,
        /*
         * ':'
         */
        VarTypeAssignment,
        /*
         * '->'
         */
        ReturnTypeAssignment,
        /*
         * ; semicolon
         */
        SemiColon,
        /*
         * // or /* */
         */
        Comment,
        /*
         * 1.102
         */
        Float,
        /*
         *   Collection
         */
        Collection {
            name: String,
            collection_type: String,
            stored_value_type_single: String,
            stored_value_type_tuple: (String, String),
        },
        /*
         * [
         */
        LeftBracket,
        /*
         * ]
         */
        RightBracket,
        /*
         * =>
         */
        FatArrow,
        /*
        Used as a bad return value
        */
        None,
    }
    impl PartialEq for TokenTypes {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (TokenTypes::FunctionCallArguments, TokenTypes::FunctionCallArguments) => true,
                (TokenTypes::SemiColon, TokenTypes::SemiColon) => true,
                (TokenTypes::Int, TokenTypes::Int) => true,
                (TokenTypes::Float, TokenTypes::Float) => true,
                (TokenTypes::String, TokenTypes::String) => true,
                (TokenTypes::Char, TokenTypes::Char) => true,
                (TokenTypes::Operator, TokenTypes::Operator) => true,
                (TokenTypes::AssignmentOperator, TokenTypes::AssignmentOperator) => true,
                (TokenTypes::LeftParenthesis, TokenTypes::LeftParenthesis) => true,
                (TokenTypes::RightParenthesis, TokenTypes::RightParenthesis) => true,
                (TokenTypes::Function, TokenTypes::Function) => true,
                (TokenTypes::FunctionCall, TokenTypes::FunctionCall) => true,
                (TokenTypes::VariableCall, TokenTypes::VariableCall) => true,
                (TokenTypes::ArgumentSeparator, TokenTypes::ArgumentSeparator) => true,
                (TokenTypes::Assignment, TokenTypes::Assignment) => true,
                (TokenTypes::VarTypeAssignment, TokenTypes::VarTypeAssignment) => true,
                (TokenTypes::RightCurly, TokenTypes::RightCurly) => true,
                (TokenTypes::LeftCurly, TokenTypes::LeftCurly) => true,
                (TokenTypes::ReturnTypeAssignment, TokenTypes::ReturnTypeAssignment) => true,
                (TokenTypes::Variable, TokenTypes::Variable) => true,

                (
                    TokenTypes::Collection {
                        name: ref name_a,
                        collection_type: ref type_a,
                        stored_value_type_single: ref stored_a,
                        stored_value_type_tuple: ref _tuple_a,
                    },
                    TokenTypes::Collection {
                        name: ref name_b,
                        collection_type: ref type_b,
                        stored_value_type_single: ref stored_b,
                        stored_value_type_tuple: ref _tuple_b,
                    },
                ) => name_a == name_b && type_a == type_b && stored_a == stored_b,

                (TokenTypes::Comment, TokenTypes::Comment) => true,
                (TokenTypes::Bool, TokenTypes::Bool) => true,
                (TokenTypes::LeftBracket, TokenTypes::LeftBracket) => true,
                (TokenTypes::RightBracket, TokenTypes::RightBracket) => true,
                (TokenTypes::FatArrow, TokenTypes::FatArrow) => true,
                (TokenTypes::None, TokenTypes::None) => true,
                _ => false,
            }
        }
    }

    impl Eq for TokenTypes {}

    impl TokenTypes {
        pub fn to_string(&self) -> String {
            match self {
                TokenTypes::FatArrow => "FatArrow".to_string(),
                TokenTypes::FunctionCallArguments => "FunctionCallArguments".to_string(),
                TokenTypes::Float => "Float".to_string(),
                TokenTypes::SemiColon => "SemiColon".to_string(),
                TokenTypes::FunctionArguments => "FunctionArguments".to_string(),
                TokenTypes::Int => "Int".to_string(),
                TokenTypes::String => "String".to_string(),
                TokenTypes::Char => "Char".to_string(),
                TokenTypes::Operator => "Operator".to_string(),
                TokenTypes::AssignmentOperator => "AssignmentOperator".to_string(),
                TokenTypes::Bool => "Bool".to_string(),
                TokenTypes::LeftParenthesis => "LeftParenthesis".to_string(),
                TokenTypes::RightParenthesis => "RightParenthesis".to_string(),
                TokenTypes::Function => "Function".to_string(),
                TokenTypes::FunctionCall => "FunctionCall".to_string(),
                TokenTypes::Variable => "Variable".to_string(),
                TokenTypes::VariableCall => "VariableCall".to_string(),
                TokenTypes::ArgumentSeparator => "ArgumentSeparator".to_string(),
                TokenTypes::Assignment => "Assignment".to_string(),
                TokenTypes::VarTypeAssignment => "VarTypeAssignment".to_string(),
                TokenTypes::RightCurly => "RightCurly".to_string(),
                TokenTypes::Collection {
                    name,
                    collection_type,
                    stored_value_type_single,
                    stored_value_type_tuple,
                } => {
                    format!(
                        "Collection: {} {} {} {:?}",
                        name, collection_type, stored_value_type_single, stored_value_type_tuple
                    )
                }
                TokenTypes::LeftCurly => "LeftCurly".to_string(),
                TokenTypes::ReturnTypeAssignment => "ReturnTypeAssignment".to_string(),
                TokenTypes::Comment => "Comment".to_string(),
                TokenTypes::RightBracket => "RightBracket".to_string(),
                TokenTypes::LeftBracket => "LeftBracket".to_string(),
                TokenTypes::None => "None".to_string(),
            }
        }
    }
}
