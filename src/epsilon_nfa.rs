use std::collections::{HashMap, HashSet};

use crate::{
    StateId, StatePair,
    nfa::{Nfa, NfaBuilder},
};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Alphabet {
    Char(char),
    Epsilon,
}
type NFATransition = HashMap<Alphabet, HashSet<StateId>>;

pub struct EpsilonNfaBuilder {
    pub transitions: Vec<NFATransition>,
}

impl EpsilonNfaBuilder {
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

    pub fn add_epsilon_transition(&mut self, from: StateId, to: StateId) {
        self.transitions[from]
            .entry(Alphabet::Epsilon)
            .or_default()
            .insert(to);
    }

    pub fn add_transition(&mut self, from: StateId, c: char, to: StateId) {
        self.transitions[from]
            .entry(Alphabet::Char(c))
            .or_default()
            .insert(to);
    }

    pub fn add_transition_range<I: IntoIterator<Item = char>>(
        &mut self,
        from: StateId,
        chars: I,
        to: StateId,
    ) {
        for c in chars {
            self.transitions[from]
                .entry(Alphabet::Char(c))
                .or_default()
                .insert(to);
        }
    }

    pub fn make_deep_copy(&mut self, start: StateId, end: StateId) -> Result<StatePair, String> {
        let mut mappings: HashMap<StateId, StateId> = HashMap::new();
        mappings.insert(start, self.add_state());

        let mut stack: Vec<StateId> = Vec::new();
        stack.push(start);
        while let Some(curr) = stack.pop() {
            for (&alphabet, next_states) in &self.transitions[curr].clone() {
                for &next in next_states {
                    match mappings.get(&next) {
                        Some(&_) => {}
                        None => {
                            mappings.insert(next, self.add_state());
                            stack.push(next);
                        }
                    };
                    match alphabet {
                        Alphabet::Char(c) => {
                            self.add_transition(mappings[&curr], c, mappings[&next]);
                        }
                        Alphabet::Epsilon => {
                            self.add_epsilon_transition(mappings[&curr], mappings[&next])
                        }
                    }
                }
            }
        }

        let Some(&mappings_end) = mappings.get(&end) else {
            return Err("Could not reach end in make_deep_copy()".to_string());
        };

        Ok((mappings[&start], mappings_end))
    }
}

#[derive(Debug, Clone)]
pub struct EpsilonNfa {
    transitions: Vec<NFATransition>,
    start: StateId,
    end: StateId,
}

impl EpsilonNfa {
    pub fn new(transitions: Vec<NFATransition>, start: StateId, end: StateId) -> Self {
        Self {
            transitions,
            start,
            end,
        }
    }

    fn epsilon_closure(&self, curr: StateId, res: &mut HashSet<StateId>) {
        res.insert(curr);
        if let Some(next_states) = self.transitions[curr].get(&Alphabet::Epsilon) {
            for &next in next_states {
                if !res.contains(&next) {
                    self.epsilon_closure(next, res);
                }
            }
        }
    }

    pub fn check(&self, input: &str) -> Result<bool, String> {
        let mut epsilon_closure_start: HashSet<StateId> = HashSet::new();
        self.epsilon_closure(self.start, &mut epsilon_closure_start);

        let mut current_states: HashSet<StateId> = HashSet::new();
        current_states.extend(epsilon_closure_start);
        for c in input.chars() {
            let mut next_states: HashSet<StateId> = HashSet::new();
            for &curr in &current_states {
                if let Some(adj) = self.transitions[curr].get(&Alphabet::Char(c)) {
                    for &next in adj {
                        let mut epsilon_closure_next: HashSet<StateId> = HashSet::new();
                        self.epsilon_closure(next, &mut epsilon_closure_next);
                        next_states.extend(epsilon_closure_next);
                    }
                }
            }
            current_states = next_states;
        }

        Ok(current_states.contains(&self.end))
    }

    pub fn to_nfa(&self) -> Nfa {
        let mut nfa_builder = NfaBuilder::new();

        let n = self.transitions.len();
        while nfa_builder.add_state() != n {}

        let nfa_start = self.start;
        let mut nfa_end: HashSet<StateId> = HashSet::new();

        let mut state_epsilon_closure: Vec<HashSet<StateId>> = Vec::new();
        state_epsilon_closure.resize(n, Default::default());
        for (curr, curr_epsilon_closure) in state_epsilon_closure.iter_mut().enumerate() {
            self.epsilon_closure(curr, curr_epsilon_closure);

            if curr_epsilon_closure.contains(&self.end) {
                nfa_end.insert(curr);
            }
        }

        for curr in 0..n {
            for &epsilon_state in &state_epsilon_closure[curr] {
                for (&alphabet, next_states) in &self.transitions[epsilon_state] {
                    if let Alphabet::Char(c) = alphabet {
                        for &next in next_states {
                            nfa_builder.transitions[curr]
                                .entry(c)
                                .or_default()
                                .extend(state_epsilon_closure[next].iter().copied());
                        }
                    }
                }
            }
        }

        Nfa::new(nfa_builder.transitions, nfa_start, nfa_end)
    }
}
