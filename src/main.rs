type StateId = usize;
type StatePair = (StateId, StateId);

use crate::regex::RegularExpression;

mod dfa;
mod epsilon_nfa;
mod minimized_dfa;
mod nfa;
mod parser;
mod regex;

fn main() {
    let pattern: &str = "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?";
    let mut re: RegularExpression = RegularExpression::new(pattern);
    re.generate();
    match re.check("1") {
        Ok(true) => {
            println!("matches");
        }
        Ok(false) => {
            println!("does not match");
        }
        Err(e) => {
            eprintln!("Error while checking: {e}");
        }
    }
}

#[cfg(test)]
mod tests;