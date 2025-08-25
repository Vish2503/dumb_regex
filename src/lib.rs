pub mod regex;

mod dfa;
mod epsilon_nfa;
mod minimized_dfa;
mod nfa;
mod parser;

type Alphabet = char;
type StateId = usize;
type StatePair = (StateId, StateId);
