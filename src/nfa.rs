use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Alphabet {
    Char(char),
}
type StateId = usize;
type NFATransition = HashMap<Alphabet, HashSet<StateId>>;

#[derive(Debug)]
pub struct Nfa {
    pub transitions: Vec<NFATransition>,
    pub start: Option<StateId>,
    pub end: Option<HashSet<StateId>>,
}

impl Nfa {
    pub fn new() -> Self {
        Self {
            transitions: Vec::new(),
            start: None,
            end: None,
        }
    }

    pub fn check(&self, input: &str) -> Result<bool, String> {
        let Some(start) = self.start else {
            return Err("Nfa start has not been initialized yet".to_string());
        };
        let Some(ref end_states) = self.end else {
            return Err("Nfa end has not been initialized yet".to_string());
        };

        let mut current_states: HashSet<StateId> = HashSet::new();
        current_states.insert(start);
        for c in input.chars() {
            let mut next_states: HashSet<StateId> = HashSet::new();
            for &curr in &current_states {
                if let Some(adj) = self.transitions[curr].get(&Alphabet::Char(c)) {
                    for &next in adj {
                        next_states.insert(next);
                    }
                }
            }
            current_states = next_states;
        }

        for end in end_states {
            if current_states.contains(end) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
