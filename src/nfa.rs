use std::collections::{BTreeSet, HashMap, HashSet};

use crate::{
    Alphabet, StateId,
    dfa::{Dfa, DfaBuilder},
};

type NFATransition = HashMap<Alphabet, HashSet<StateId>>;

pub struct NfaBuilder {
    pub transitions: Vec<NFATransition>,
}

impl NfaBuilder {
    pub fn new() -> Self {
        Self {
            transitions: Vec::new(),
        }
    }

    pub fn add_state(&mut self) -> StateId {
        let state: StateId = self.transitions.len();
        self.transitions.push(NFATransition::new());
        state
    }
}

#[derive(Debug)]
pub struct Nfa {
    transitions: Vec<NFATransition>,
    start: StateId,
    end: HashSet<StateId>,
}

impl Nfa {
    pub fn new(transitions: Vec<NFATransition>, start: StateId, end: HashSet<StateId>) -> Self {
        Self {
            transitions,
            start,
            end,
        }
    }

    pub fn is_match(&self, input: &str) -> bool {
        let mut current_states: HashSet<StateId> = HashSet::new();
        current_states.insert(self.start);
        for c in input.chars() {
            let mut next_states: HashSet<StateId> = HashSet::new();
            for &curr in &current_states {
                if let Some(adj) = self.transitions[curr].get(&c) {
                    for &next in adj {
                        next_states.insert(next);
                    }
                }
            }
            current_states = next_states;
        }

        for end in &self.end {
            if current_states.contains(end) {
                return true;
            }
        }
        false
    }

    pub fn to_dfa(&self) -> Dfa {
        let mut dfa_builder = DfaBuilder::new();

        dfa_builder.add_state();

        let mut subset_to_dfa_state: HashMap<BTreeSet<StateId>, StateId> = HashMap::new();

        let dfa_start = dfa_builder.add_state();
        let start: BTreeSet<StateId> = BTreeSet::from([self.start]);
        subset_to_dfa_state.insert(start.clone(), dfa_start);

        let mut stack: Vec<BTreeSet<StateId>> = Vec::new();
        stack.push(start);
        while let Some(curr_states) = stack.pop() {
            let &curr_dfa_state = subset_to_dfa_state.get(&curr_states).expect(
                "curr_states should always be in subset_to_dfa_state due to a previous iteration",
            );

            let mut current_transitions: HashMap<Alphabet, BTreeSet<StateId>> = HashMap::new();
            for curr in curr_states {
                for (&alphabet, next_states) in &self.transitions[curr] {
                    current_transitions
                        .entry(alphabet)
                        .or_default()
                        .extend(next_states);
                }
            }

            for (alphabet, next_states) in current_transitions {
                let next_dfa_state = match subset_to_dfa_state.get(&next_states) {
                    Some(&dfa_state) => dfa_state,
                    None => {
                        let state = dfa_builder.add_state();
                        subset_to_dfa_state.insert(next_states.clone(), state);
                        stack.push(next_states);
                        state
                    }
                };
                dfa_builder.transitions[curr_dfa_state].insert(alphabet, next_dfa_state);
            }
        }

        let mut dfa_end: HashSet<StateId> = HashSet::new();
        for (subset, &dfa_state) in &subset_to_dfa_state {
            for &end in &self.end {
                if subset.contains(&end) {
                    dfa_end.insert(dfa_state);
                    break;
                }
            }
        }

        Dfa::new(dfa_builder.transitions, dfa_start, dfa_end)
    }
}
