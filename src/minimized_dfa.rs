use std::collections::{HashMap, HashSet};

use crate::{Alphabet, StateId};

type DFATransition = HashMap<Alphabet, StateId>;

#[derive(Debug)]
pub struct MinimizedDfaBuilder {
    pub transitions: Vec<DFATransition>,
}

impl MinimizedDfaBuilder {
    pub fn new() -> Self {
        Self {
            transitions: Vec::new(),
        }
    }

    pub fn add_state(&mut self) -> StateId {
        let state: StateId = self.transitions.len();
        self.transitions.push(DFATransition::new());
        state
    }
}

#[derive(Debug)]
pub struct MinimizedDfa {
    transitions: Vec<DFATransition>,
    start: StateId,
    end: HashSet<StateId>,
}

impl MinimizedDfa {
    pub fn new(transitions: Vec<DFATransition>, start: StateId, end: HashSet<StateId>) -> Self {
        Self {
            transitions,
            start,
            end,
        }
    }

    pub fn check(&self, input: &str) -> Result<bool, String> {
        let mut curr: StateId = self.start;
        for c in input.chars() {
            if let Some(&next) = self.transitions[curr].get(&c) {
                curr = next;
            } else {
                return Ok(false);
            }
        }

        Ok(self.end.contains(&curr))
    }
}
