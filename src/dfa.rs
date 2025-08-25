use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{
    Alphabet, StateId, StatePair,
    minimized_dfa::{MinimizedDfa, MinimizedDfaBuilder},
};

type DFATransition = HashMap<Alphabet, StateId>;

pub struct DfaBuilder {
    pub transitions: Vec<DFATransition>,
}

impl DfaBuilder {
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
pub struct Dfa {
    transitions: Vec<DFATransition>,
    start: StateId,
    end: HashSet<StateId>,
}

impl Dfa {
    pub fn new(transitions: Vec<DFATransition>, start: StateId, end: HashSet<StateId>) -> Self {
        Self {
            transitions,
            start,
            end,
        }
    }

    pub fn is_match(&self, input: &str) -> bool {
        let mut curr: StateId = self.start;
        for c in input.chars() {
            if let Some(&next) = self.transitions[curr].get(&c) {
                curr = next;
            } else {
                return false;
            }
        }

        self.end.contains(&curr)
    }

    pub fn to_minimized_dfa(&self) -> MinimizedDfa {
        let mut minimized_dfa_builder = MinimizedDfaBuilder::new();

        let total_dfa_states = self.transitions.len();

        let mut reachable_states: HashSet<StateId> = HashSet::from([self.start]);
        let mut current_states: HashSet<StateId> = HashSet::from([self.start]);
        while !current_states.is_empty() {
            let mut next_states: HashSet<StateId> = HashSet::new();
            for &state in &current_states {
                for (&_alphabet, &next) in &self.transitions[state] {
                    if !reachable_states.contains(&next) {
                        next_states.insert(next);
                    }
                }
            }
            reachable_states.extend(next_states.iter().copied());
            current_states = next_states;
        }

        let mut dead_states: HashSet<StateId> = HashSet::new();
        for i in 0..total_dfa_states {
            let mut current_states: HashSet<StateId> = HashSet::from([i]);

            let mut end_state_reachable = false;
            let mut stack: Vec<StateId> = Vec::from([i as StateId]);
            while let Some(curr) = stack.pop() {
                if self.end.contains(&curr) {
                    end_state_reachable = true;
                    break;
                }
                for (&_alphabet, &next) in &self.transitions[curr] {
                    if !current_states.contains(&next) {
                        current_states.insert(next);
                        stack.push(next);
                    }
                }
            }

            if !end_state_reachable {
                dead_states.insert(i as StateId);
            }
        }

        let mut group_mapping: HashMap<StateId, StateId> = HashMap::new();
        group_mapping.insert(0, 0);
        for i in 0..total_dfa_states {
            if !reachable_states.contains(&i) || dead_states.contains(&i) {
                continue;
            }

            if self.end.contains(&i) {
                group_mapping.insert(i, 1);
            } else {
                group_mapping.insert(i, 2);
            }
        }

        loop {
            let mut change = false;
            for c in u8::MIN..=u8::MAX {
                let c = c as char;
                let mut group_to_states: BTreeMap<StatePair, HashSet<StateId>> = BTreeMap::new();
                for &curr in group_mapping.keys() {
                    let next = match self.transitions[curr].get(&c) {
                        Some(&next) => next,
                        None => 0,
                    };
                    group_to_states
                        .entry((group_mapping[&curr], group_mapping[&next]))
                        .or_default()
                        .insert(curr);
                }

                let mut new_group_mapping: HashMap<StateId, StateId> = HashMap::new();
                for (group_number, (&_, states)) in group_to_states.iter().enumerate() {
                    for &state in states {
                        new_group_mapping.insert(state, group_number);
                    }
                }

                if new_group_mapping != group_mapping {
                    group_mapping = new_group_mapping;
                    change = true;
                    break;
                }
            }

            if !change {
                break;
            }
        }

        minimized_dfa_builder.add_state();

        let largest_node = group_mapping
            .values()
            .max()
            .expect("there must be some nodes in minimized_dfa_builder");

        while minimized_dfa_builder.add_state() != *largest_node {}

        let mut minimized_dfa_start = 0;
        let mut minimized_dfa_end: HashSet<StateId> = HashSet::new();
        for (&dfa_state, &group) in &group_mapping {
            for c in u8::MIN..=u8::MAX {
                let c = c as char;
                if let Some(next_dfa_state) = self.transitions[dfa_state].get(&c) {
                    minimized_dfa_builder.transitions[group]
                        .insert(c, group_mapping[next_dfa_state]);
                }
            }

            if dfa_state == self.start {
                minimized_dfa_start = group;
            }

            if self.end.contains(&dfa_state) {
                minimized_dfa_end.insert(group);
            }
        }

        MinimizedDfa::new(
            minimized_dfa_builder.transitions,
            minimized_dfa_start,
            minimized_dfa_end,
        )
    }
}
