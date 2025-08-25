use crate::{epsilon_nfa::EpsilonNfa, parser::Parser};

pub struct RegularExpression {
    pattern: String,
}

impl RegularExpression {
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }

    pub fn to_epsilon_nfa(&self) -> Result<EpsilonNfa, String> {
        let mut parser = Parser::new(self.pattern.as_str());
        parser.parse()
    }
}
