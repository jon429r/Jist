#[cfg(test)]
mod tokenizer_tests {
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::statement_tokenizer::tokenizer::tokenizers::{self, tokenize};
    use crate::token_type::token_types::TokenTypes;

    #[test]
    fn test_tokenize_collection_assignment_dict() {
        // Input for testing
        let input = r#"let a: dict<string, int> = {"One" => 1, "Two" => 2, "Three" => 3};"#;

        // Call the function to tokenize the input
        let result = tokenize(input.to_string());

        // Expected output tokens
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Collection {
                    name: "a".to_string(),
                    collection_type: "dict".to_string(),
                    stored_value_type_single: "".to_string(),
                    stored_value_type_tuple: ("string".to_string(), "int".to_string()),
                },
                chars_read: 25,
                value: "name: a collection_type: dict<string, int>".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::LeftCurly,
                chars_read: 1,
                value: "{".to_string(),
            },
            ParseInfo {
                token: TokenTypes::String,
                chars_read: 5,
                value: "\"One\"".to_string(),
            },
            ParseInfo {
                token: TokenTypes::FatArrow,
                chars_read: 2,
                value: "=>".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "1".to_string(),
            },
            ParseInfo {
                token: TokenTypes::ArgumentSeparator,
                chars_read: 1,
                value: ",".to_string(),
            },
            ParseInfo {
                token: TokenTypes::String,
                chars_read: 5,
                value: "\"Two\"".to_string(),
            },
            ParseInfo {
                token: TokenTypes::FatArrow,
                chars_read: 2,
                value: "=>".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "2".to_string(),
            },
            ParseInfo {
                token: TokenTypes::ArgumentSeparator,
                chars_read: 1,
                value: ",".to_string(),
            },
            ParseInfo {
                token: TokenTypes::String,
                chars_read: 7,
                value: "\"Three\"".to_string(),
            },
            ParseInfo {
                token: TokenTypes::FatArrow,
                chars_read: 2,
                value: "=>".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "3".to_string(),
            },
            ParseInfo {
                token: TokenTypes::RightCurly,
                chars_read: 1,
                value: "}".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];

        // Assert the result matches the expected output
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_collection_assignment_dict_bool_string() {
        // Input for testing
        let input = r#"let c: dict<bool, string> = {true => "Yes", false => "No"};"#;

        // Call the function to tokenize the input
        let result = tokenize(input.to_string());

        // Expected output tokens
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Collection {
                    name: "c".to_string(),
                    collection_type: "dict".to_string(),
                    stored_value_type_single: "".to_string(),
                    stored_value_type_tuple: ("bool".to_string(), "string".to_string()),
                },
                chars_read: 26,
                value: "name: c collection_type: dict<bool, string>".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::LeftCurly,
                chars_read: 1,
                value: "{".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Bool,
                chars_read: 4,
                value: "true".to_string(),
            },
            ParseInfo {
                token: TokenTypes::FatArrow,
                chars_read: 2,
                value: "=>".to_string(),
            },
            ParseInfo {
                token: TokenTypes::String,
                chars_read: 5,
                value: "\"Yes\"".to_string(),
            },
            ParseInfo {
                token: TokenTypes::ArgumentSeparator,
                chars_read: 1,
                value: ",".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Bool,
                chars_read: 5,
                value: "false".to_string(),
            },
            ParseInfo {
                token: TokenTypes::FatArrow,
                chars_read: 2,
                value: "=>".to_string(),
            },
            ParseInfo {
                token: TokenTypes::String,
                chars_read: 4,
                value: "\"No\"".to_string(),
            },
            ParseInfo {
                token: TokenTypes::RightCurly,
                chars_read: 1,
                value: "}".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];

        // Assert the result matches the expected output
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_collection_assignment_dict_char_float() {
        // Input for testing
        let input = r#"let b: dict<char, float> = {'A' => 1.1, 'B' => 2.2, 'C' => 3.3};"#;

        // Call the function to tokenize the input
        let result = tokenize(input.to_string());

        // Expected output tokens
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Collection {
                    name: "b".to_string(),
                    collection_type: "dict".to_string(),
                    stored_value_type_single: "".to_string(),
                    stored_value_type_tuple: ("char".to_string(), "float".to_string()),
                },
                chars_read: 25,
                value: "name: b collection_type: dict<char, float>".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::LeftCurly,
                chars_read: 1,
                value: "{".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Char,
                chars_read: 3,
                value: "'A'".to_string(),
            },
            ParseInfo {
                token: TokenTypes::FatArrow,
                chars_read: 2,
                value: "=>".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Float,
                chars_read: 3,
                value: "1.1".to_string(),
            },
            ParseInfo {
                token: TokenTypes::ArgumentSeparator,
                chars_read: 1,
                value: ",".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Char,
                chars_read: 3,
                value: "'B'".to_string(),
            },
            ParseInfo {
                token: TokenTypes::FatArrow,
                chars_read: 2,
                value: "=>".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Float,
                chars_read: 3,
                value: "2.2".to_string(),
            },
            ParseInfo {
                token: TokenTypes::ArgumentSeparator,
                chars_read: 1,
                value: ",".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Char,
                chars_read: 3,
                value: "'C'".to_string(),
            },
            ParseInfo {
                token: TokenTypes::FatArrow,
                chars_read: 2,
                value: "=>".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Float,
                chars_read: 3,
                value: "3.3".to_string(),
            },
            ParseInfo {
                token: TokenTypes::RightCurly,
                chars_read: 1,
                value: "}".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];

        // Assert the result matches the expected output
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_declaration_int() {
        let input = "let a: int = 1;".to_string();

        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 10,
                value: "int".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "1".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];

        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_declaration_float() {
        let input = "let a: float = 1.102;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 12,
                value: "float".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Float,
                chars_read: 5,
                value: "1.102".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_declaration_string() {
        let input = "let a: string = \"Hello, World!\";".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 13,
                value: "string".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::String,
                chars_read: 15,
                value: "\"Hello, World!\"".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_declaration_char() {
        let input = "let a: char = 'a';".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 11,
                value: "char".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Char,
                chars_read: 3,
                value: "'a'".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_declaration_boolean_true() {
        let input = "let a: bool = True;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 11,
                value: "bool".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Bool,
                chars_read: 4,
                value: "True".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_declaration_boolean_false() {
        let input = "let a: bool = False;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 11,
                value: "bool".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Bool,
                chars_read: 5,
                value: "False".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_call_int() {
        let input = "a = 1;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::VariableCall,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "1".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_call_float() {
        let input = "a = 1.102;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::VariableCall,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Float,
                chars_read: 5,
                value: "1.102".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_call_string() {
        let input = "a = \"Hello, World!\";".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::VariableCall,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::String,
                chars_read: 15,
                value: "\"Hello, World!\"".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_call_char() {
        let input = "a = 'a';".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::VariableCall,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Char,
                chars_read: 3,
                value: "'a'".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_call_boolean() {
        let input = "a = True;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::VariableCall,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Bool,
                chars_read: 4,
                value: "True".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_function_call() {
        let input = "add(1, 2);".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::FunctionCall,
                chars_read: 3,
                value: "add".to_string(),
            },
            ParseInfo {
                token: TokenTypes::LeftParenthesis,
                chars_read: 1,
                value: "(".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "1".to_string(),
            },
            ParseInfo {
                token: TokenTypes::ArgumentSeparator,
                chars_read: 1,
                value: ",".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "2".to_string(),
            },
            ParseInfo {
                token: TokenTypes::RightParenthesis,
                chars_read: 1,
                value: ")".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_operation() {
        let input = "1 + 2;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "1".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Operator,
                chars_read: 1,
                value: "+".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "2".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_operation_with_parethisis() {
        let input = "(1 + 2);".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::LeftParenthesis,
                chars_read: 1,
                value: "(".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "1".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Operator,
                chars_read: 1,
                value: "+".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "2".to_string(),
            },
            ParseInfo {
                token: TokenTypes::RightParenthesis,
                chars_read: 1,
                value: ")".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }
}
