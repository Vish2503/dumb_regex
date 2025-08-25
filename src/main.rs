use dumb_regex::regex::RegularExpression;

fn main() -> Result<(), String> {
    let pattern = String::from("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let re = RegularExpression::new(pattern)
        .to_epsilon_nfa()?
        .to_nfa()
        .to_dfa()
        .to_minimized_dfa();

    if re.is_match("3.14159") {
        println!("matches");
    } else {
        println!("does not match");
    }

    Ok(())
}
