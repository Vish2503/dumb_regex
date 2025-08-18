use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Alphabet {
    Char(char),
    Epsilon,
}
type StateId = usize;
type StatePair = (StateId, StateId);
type NFATransition = HashMap<Alphabet, HashSet<StateId>>;

#[derive(Debug, Clone)]
pub struct EpsilonNfa {
    pub transitions: Vec<NFATransition>,
    pub start: Option<StateId>,
    pub end: Option<StateId>,
}

impl EpsilonNfa {
    pub fn new() -> Self {
        Self {
            transitions: Vec::new(),
            start: None,
            end: None,
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

    pub fn epsilon_closure(&self, curr: StateId, res: &mut HashSet<StateId>) {
        res.insert(curr);
        if let Some(next_states) = self.transitions[curr].get(&Alphabet::Epsilon) {
            for &next in next_states {
                if !res.contains(&next) {
                    self.epsilon_closure(next, res);
                }
            }
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

    pub fn check(&self, input: &str) -> Result<bool, String> {
        let Some(start) = self.start else {
            return Err("Epsilon Nfa start has not been initialized yet".to_string());
        };
        let Some(end) = self.end else {
            return Err("Epsilon Nfa end has not been initialized yet".to_string());
        };

        let mut epsilon_closure_start: HashSet<StateId> = HashSet::new();
        self.epsilon_closure(start, &mut epsilon_closure_start);

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

        Ok(current_states.contains(&end))
    }
}
