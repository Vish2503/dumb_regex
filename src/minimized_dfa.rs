use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Alphabet {
    Char(char),
}
type StateId = usize;
type DFATransition = HashMap<Alphabet, StateId>;

#[derive(Debug)]
pub struct MinimizedDfa {
    pub transitions: Vec<DFATransition>,
    pub start: Option<StateId>,
    pub end: Option<HashSet<StateId>>,
}

impl MinimizedDfa {
    pub fn new() -> Self {
        Self {
            transitions: Vec::new(),
            start: None,
            end: None,
        }
    }

    pub fn add_state(&mut self) -> StateId {
        let state: StateId = self.transitions.len();
        self.transitions.push(DFATransition::new());
        state
    }

    pub fn check(&self, input: &str) -> Result<bool, String> {
        let Some(start) = self.start else {
            return Err("Nfa start has not been initialized yet".to_string());
        };
        let Some(ref end_states) = self.end else {
            return Err("Nfa end has not been initialized yet".to_string());
        };

        let mut curr: StateId = start;
        for c in input.chars() {
            if let Some(&next) = self.transitions[curr].get(&Alphabet::Char(c)) {
                curr = next;
            } else {
                return Ok(false);
            }
        }

        Ok(end_states.contains(&curr))
    }
}
