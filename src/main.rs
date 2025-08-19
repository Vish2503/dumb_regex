use crate::regex::RegularExpression;

type Alphabet = char;
type StateId = usize;
type StatePair = (StateId, StateId);

mod dfa;
mod epsilon_nfa;
mod minimized_dfa;
mod nfa;
mod parser;
mod regex;

#[cfg(test)]
mod tests;

fn main() -> Result<(), String> {
    let pattern: &str = "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?";
    let re: RegularExpression = RegularExpression::new(pattern);
    let epsilon_nfa = re.to_epsilon_nfa();
    match epsilon_nfa.check("1") {
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
    let nfa = epsilon_nfa.to_nfa();
    match nfa.check("1") {
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
    let dfa = nfa.to_dfa();
    match dfa.check("1") {
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
    let minimized_dfa = dfa.to_minimized_dfa();
    match minimized_dfa.check("1") {
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
    Ok(())
}
