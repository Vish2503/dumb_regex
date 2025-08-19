use std::process::exit;

use crate::{epsilon_nfa::EpsilonNfa, parser::Parser};

pub struct RegularExpression {
    pattern: String,
}

impl RegularExpression {
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_owned(),
        }
    }

    pub fn to_epsilon_nfa(&self) -> EpsilonNfa {
        let mut parser = Parser::new(self.pattern.as_str());
        match parser.parse() {
            Ok(epsilon_nfa) => epsilon_nfa,
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        }
    }
}
