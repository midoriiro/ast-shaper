use crate::utils::name_conventions::NamingConventionType::{CamelCase, KebabCase, PascalCase, ScreamingSnakeCase, SnakeCase};
use itertools::Itertools;

enum NamingConventionType {
    CamelCase,
    PascalCase,
    KebabCase,
    SnakeCase,
    ScreamingSnakeCase,
}

pub struct NamingConventions {
    pub(crate) tokens: Vec<String>,
}

impl NamingConventions {
    pub fn parse(value: impl Into<String>) -> NamingConventions {
        let value = value.into();
        let mut chars = value.chars();
        let have_underscore = value.contains("_");
        let have_hyphen = value.contains("-");
        let have_uppercase = chars.next().unwrap().is_uppercase();
        let name_convention_type = match (have_underscore, have_hyphen, have_uppercase) {
            (false, false, true) => {
                CamelCase
            },
            (false, false, false) => {
                PascalCase
            },
            (false, true, false) => {
                KebabCase
            },
            (true, false, false) => {
                SnakeCase
            },
            (true, false, true) => {
                ScreamingSnakeCase
            },
            _ => panic!("Cannot determine naming convention type"),
        };
        let tokens = match name_convention_type {
            CamelCase | PascalCase => {
                let chars = value.chars();
                let mut tokens = Vec::new();
                let mut start_index = 0;
                let mut end_index = 0;
                for (index, char) in chars.enumerate() {
                    if index == 0 {
                        continue;
                    }
                    if index == value.len() -1 {
                        end_index += 1;
                    }
                    if char.is_uppercase() || index == value.len() - 1 {
                        let token = &value[start_index..end_index + 1];
                        tokens.push(token.to_string());
                        start_index = index;
                        end_index = index;
                        continue;
                    }
                    end_index += 1;
                }
                tokens
            }
            KebabCase => {
                value.split("-")
                    .map(|token| token.to_string())
                    .collect::<Vec<_>>()
            }
            SnakeCase | ScreamingSnakeCase => {
                value.split("_")
                    .map(|token| token.to_string())
                    .collect::<Vec<_>>()
            }
        };
        Self {
            tokens: tokens.into_iter()
                .map(|token| token.to_lowercase())
                .collect(),
        }
    }
}

impl NamingConventions {
    pub fn to_camel_case(value: impl Into<String>) -> String {
        let mut tokens = NamingConventions::parse(value).tokens;
        for token in tokens.iter_mut() {
            let mut char = token.get_mut(0..1);
            let char = char.as_mut().unwrap();
            char.make_ascii_uppercase();
        }
        tokens.join("")
    }
    
    pub fn to_pascal_case(value: impl Into<String>) -> String {
        let mut tokens = NamingConventions::parse(value).tokens;
        for token in tokens[1..].iter_mut() {
            let mut char = token.get_mut(0..1);
            let char = char.as_mut().unwrap();
            char.make_ascii_uppercase();
        }
        tokens.join("")
    }

    pub fn to_kebab_case(value: impl Into<String>) -> String {
        let mut tokens = NamingConventions::parse(value).tokens;
        tokens.join("-")
    }

    pub fn to_snake_case(value: impl Into<String>) -> String {
        let mut tokens = NamingConventions::parse(value).tokens;
        tokens.join("_")
    }
    
    pub fn to_screaming_snake_case(value: impl Into<String>) -> String {
        let mut tokens = NamingConventions::parse(value).tokens;
        for token in tokens.iter_mut() {
            token.make_ascii_uppercase();
        }
        tokens.join("_")
    }
}

impl <T: Into<String>> From<T> for NamingConventions {
    fn from(value: T) -> Self {
        Self::parse(value)
    }
}

