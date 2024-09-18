pub mod variables {
    use super::variable::Variable;
    // use super::base_variables::BaseVariables::{Pi, E};

    pub static mut VARIABLE_STACK: Vec<Variable> = Vec::new();
}

pub mod variable {
    use super::base_types::BaseTypes;
    use super::variables::VARIABLE_STACK;

    #[derive(Debug, Clone)]
    pub struct Variable {
        pub name: String,
        pub value: BaseTypes,
        pub var_type: BaseTypes,
    }

    pub trait get_value {
        fn get_value(&self) -> BaseTypes;
    }

    impl PartialEq for BaseTypes {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (BaseTypes::Int(x), BaseTypes::Int(y)) => x == y,
                (BaseTypes::Float(x), BaseTypes::Float(y)) => x == y,
                (BaseTypes::StringWrapper(s1), BaseTypes::StringWrapper(s2)) => s1 == s2,
                (BaseTypes::Char(c1), BaseTypes::Char(c2)) => c1 == c2,
                _ => false,
            }
        }
    }

    impl Variable {
        pub fn new(name: String, value: BaseTypes, var_type: BaseTypes) -> Variable
where
            //T: Into<BaseTypes>, // Trait to ensure the value can be converted to BaseTypes
        {
            //print!("Creating new variable: {} with value: {:?} and type: {:?}\n", name, value, var_type);
            let value_as_base_type: BaseTypes = value.into();

            //let base Type int become a f64
            let var_type = match var_type {
                BaseTypes::Int(i) => BaseTypes::Float(i as f64),
                _ => var_type,
            };

            // Ensure the value type matches the variable type
            let checked_value = match var_type {
                BaseTypes::Int(_) => match value_as_base_type {
                    BaseTypes::Int(_) => value_as_base_type,
                    _ => {
                        println!(
                            "Warning: Value type mismatch for '{}'. Setting default Int value.",
                            name
                        );
                        BaseTypes::Int(0)
                    }
                },
                BaseTypes::Float(_) => {
                    match value_as_base_type {
                        BaseTypes::Float(_) => value_as_base_type,
                        _ => {
                            println!("Warning: Value type mismatch for '{}'. Setting default Float value.", name);
                            BaseTypes::Float(0.0)
                        }
                    }
                }
                BaseTypes::StringWrapper(_) => {
                    match value_as_base_type {
                        BaseTypes::StringWrapper(_) => value_as_base_type,
                        _ => {
                            println!("Warning: Value type mismatch for '{}'. Setting default String value.", name);
                            BaseTypes::StringWrapper(String::new())
                        }
                    }
                }
                BaseTypes::Bool(_) => {
                    match value_as_base_type {
                        BaseTypes::Bool(_) => value_as_base_type,
                        _ => {
                            println!("Warning: Value type mismatch for '{}'. Setting default Bool value.", name);
                            BaseTypes::Bool(false)
                        }
                    }
                }
                BaseTypes::Char(_) => {
                    match value_as_base_type {
                        BaseTypes::Char(_) => value_as_base_type,
                        _ => {
                            println!("Warning: Value type mismatch for '{}'. Setting default Char value.", name);
                            BaseTypes::Char('\0')
                        }
                    }
                }
                BaseTypes::Null => {
                    println!(
                        "Warning: Value type mismatch for '{}'. Null type cannot have a value.",
                        name
                    );
                    BaseTypes::Null
                }
            };

            // Add to VARIABLE_STACK
            unsafe {
                VARIABLE_STACK.push(Variable {
                    name: name.clone(),
                    value: checked_value.clone(),
                    var_type: var_type.clone(),
                });
            }

            Variable {
                name,
                value: checked_value,
                var_type,
            }
        }

        pub fn set_value<T>(&mut self, value: T)
        where
            T: Into<BaseTypes>,
        {
            self.value = value.into();
        }

        pub fn get_value(&self) -> &BaseTypes {
            &self.value
        }

        pub fn get_type(&self) -> &BaseTypes {
            &self.var_type
        }

        pub fn print(&self) {
            println!("Variable Name: {}", self.name);
            println!("Variable Type: {:?}", self.var_type);
            println!("Variable Value: {:?}", self.value);
        }
    }

    impl From<i32> for BaseTypes {
        fn from(value: i32) -> Self {
            BaseTypes::Int(value)
        }
    }

    impl From<f64> for BaseTypes {
        fn from(value: f64) -> Self {
            BaseTypes::Float(value.into())
        }
    }

    impl From<String> for BaseTypes {
        fn from(value: String) -> Self {
            BaseTypes::StringWrapper(value)
        }
    }

    impl From<&str> for BaseTypes {
        fn from(value: &str) -> Self {
            BaseTypes::StringWrapper(value.to_string())
        }
    }

    impl From<bool> for BaseTypes {
        fn from(value: bool) -> Self {
            BaseTypes::Bool(value)
        }
    }

    impl From<char> for BaseTypes {
        fn from(value: char) -> Self {
            BaseTypes::Char(value)
        }
    }

    impl From<()> for BaseTypes {
        fn from(_: ()) -> Self {
            BaseTypes::Null
        }
    }

    impl From<BaseTypes> for i32 {
        fn from(value: BaseTypes) -> Self {
            match value {
                BaseTypes::Int(i) => i,
                _ => 0,
            }
        }
    }

    impl From<BaseTypes> for f64 {
        fn from(value: BaseTypes) -> Self {
            match value {
                BaseTypes::Float(f) => f,
                _ => 0.0,
            }
        }
    }

    impl From<BaseTypes> for String {
        fn from(value: BaseTypes) -> Self {
            match value {
                BaseTypes::StringWrapper(s) => s,
                _ => String::new(),
            }
        }
    }

    impl From<BaseTypes> for bool {
        fn from(value: BaseTypes) -> Self {
            match value {
                BaseTypes::Bool(b) => b,
                _ => false,
            }
        }
    }

    impl From<BaseTypes> for char {
        fn from(value: BaseTypes) -> Self {
            match value {
                BaseTypes::Char(c) => c,
                _ => '\0',
            }
        }
    }

    impl From<BaseTypes> for () {
        fn from(_: BaseTypes) -> Self {
            ()
        }
    }

    impl ToString for BaseTypes {
        fn to_string(&self) -> String {
            match self {
                BaseTypes::Int(i) => i.to_string(),
                BaseTypes::Float(f) => f.to_string(),
                BaseTypes::StringWrapper(s) => s.to_string(),
                BaseTypes::Bool(b) => b.to_string(),
                BaseTypes::Char(c) => c.to_string(),
                BaseTypes::Null => String::from("null"),
            }
        }
    }
}

pub mod base_variables {
    use super::base_types::BaseTypes;
    // Stores some basic variables that most PLs have like pi, e, etc.
    #[derive(Debug, Clone)]
    pub enum BaseVariables {
        Pi,
        E,
    }

    pub struct Pi {
        pub value: f64,
    }

    impl Pi {
        pub fn new() -> Pi {
            Pi {
                value: std::f64::consts::PI,
            }
        }

        pub fn get_value(&self) -> f64 {
            self.value
        }

        pub fn get_type(&self) -> BaseTypes {
            BaseTypes::Float(self.value)
        }

        pub fn print(&self) {
            println!("Pi: {}", self.value);
        }
    }

    pub struct E {
        pub value: f64,
    }

    impl E {
        pub fn new() -> E {
            E {
                value: std::f64::consts::E,
            }
        }

        pub fn get_value(&self) -> f64 {
            self.value
        }

        pub fn get_type(&self) -> BaseTypes {
            BaseTypes::Float(self.value)
        }

        pub fn print(&self) {
            println!("E: {}", self.value);
        }
    }
}

pub mod base_types {
    #[derive(Debug, Clone)]
    pub enum BaseTypes {
        Int(i32),
        Float(f64),
        StringWrapper(String),
        Bool(bool),
        Char(char),
        Null,
    }

    pub struct Int {
        pub value: i32,
    }

    impl Int {
        pub fn new(value: i32) -> Int {
            Int { value: value }
        }
    }

    pub struct Float {
        pub value: f64,
    }

    impl Float {
        pub fn new(value: f64) -> Float {
            Float { value: value }
        }
    }

    pub struct StringWrapper {
        pub value: String,
    }

    impl StringWrapper {
        pub fn new(value: String) -> StringWrapper {
            StringWrapper { value: value }
        }
    }

    pub struct Char {
        pub value: char,
    }

    impl Char {
        pub fn new(value: char) -> Char {
            Char { value: value }
        }
    }

    pub struct Bool {
        pub value: bool,
    }

    impl Bool {
        pub fn new(value: bool) -> Bool {
            Bool { value: value }
        }
    }

    pub struct Null {
        pub value: (),
    }

    impl Null {
        pub fn new() -> Null {
            Null { value: () }
        }
    }
}
