use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    process::exit,
};

use crate::{
    StateId, StatePair,
    dfa::{self, Dfa},
    epsilon_nfa::{self, EpsilonNfa},
    minimized_dfa::{self, MinimizedDfa},
    nfa::{self, Nfa},
    parser::Parser,
};

pub struct RegularExpression<'a> {
    parser: Parser<'a>,
    epsilon_nfa: Option<EpsilonNfa>,
    nfa: Nfa,
    dfa: Dfa,
    minimized_dfa: MinimizedDfa,
}

impl<'a> RegularExpression<'a> {
    pub fn new(pattern: &'a str) -> Self {
        Self {
            parser: Parser::new(pattern),
            epsilon_nfa: None,
            nfa: Nfa::new(),
            dfa: Dfa::new(),
            minimized_dfa: MinimizedDfa::new(),
        }
    }

    fn generate_epsilon_nfa(&mut self) {
        match self.parser.parse() {
            Ok(epsilon_nfa) => {
                self.epsilon_nfa = epsilon_nfa;
            }
            Err(e) => {
                eprintln!("Error generating epsilon nfa: {e}");
                exit(1);
            }
        }
    }

    fn generate_nfa(&mut self) {
        self.generate_epsilon_nfa();

        let epsilon_nfa_start = self
            .epsilon_nfa
            .as_ref()
            .unwrap()
            .start
            .expect("Epsilon Nfa should be generated already");
        let epsilon_nfa_end = self
            .epsilon_nfa
            .as_ref()
            .unwrap()
            .end
            .expect("Epsilon Nfa should be generated already");

        let n = self.epsilon_nfa.as_ref().unwrap().transitions.len();
        self.nfa.transitions.resize(n, Default::default());

        let start = epsilon_nfa_start;
        let mut end: HashSet<StateId> = HashSet::new();

        let mut state_epsilon_closure: Vec<HashSet<StateId>> = Vec::new();
        state_epsilon_closure.resize(n, Default::default());
        for (curr, curr_epsilon_closure) in state_epsilon_closure.iter_mut().enumerate() {
            self.epsilon_nfa
                .as_ref()
                .unwrap()
                .epsilon_closure(curr, curr_epsilon_closure);

            if curr_epsilon_closure.contains(&epsilon_nfa_end) {
                end.insert(curr);
            }
        }

        for curr in 0..n {
            for &epsilon_state in &state_epsilon_closure[curr] {
                for (&alphabet, next_states) in
                    &self.epsilon_nfa.as_ref().unwrap().transitions[epsilon_state]
                {
                    if let epsilon_nfa::Alphabet::Char(c) = alphabet {
                        for &next in next_states {
                            self.nfa.transitions[curr]
                                .entry(nfa::Alphabet::Char(c))
                                .or_default()
                                .extend(state_epsilon_closure[next].iter().copied());
                        }
                    }
                }
            }
        }

        self.nfa.start = Some(start);
        self.nfa.end = Some(end);
    }

    fn generate_dfa(&mut self) {
        self.generate_nfa();

        let nfa_start = self.nfa.start.expect("Nfa should be generated already");
        let nfa_end = self
            .nfa
            .end
            .as_ref()
            .expect("Nfa should be generated already");

        self.dfa.add_state();

        let mut subset_to_dfa_state: HashMap<BTreeSet<StateId>, StateId> = HashMap::new();

        let dfa_start = self.dfa.add_state();
        let start: BTreeSet<StateId> = BTreeSet::from([nfa_start]);
        subset_to_dfa_state.insert(start.clone(), dfa_start);

        let mut stack: Vec<BTreeSet<StateId>> = Vec::new();
        stack.push(start);
        while let Some(curr_states) = stack.pop() {
            let &curr_dfa_state = subset_to_dfa_state.get(&curr_states).expect(
                "curr_states should always be in subset_to_dfa_state due to a previous iteration",
            );

            let mut current_transitions: HashMap<nfa::Alphabet, BTreeSet<StateId>> = HashMap::new();
            for curr in curr_states {
                for (&alphabet, next_states) in &self.nfa.transitions[curr] {
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
                        let state = self.dfa.add_state();
                        subset_to_dfa_state.insert(next_states.clone(), state);
                        stack.push(next_states);
                        state
                    }
                };
                match alphabet {
                    nfa::Alphabet::Char(c) => {
                        self.dfa.transitions[curr_dfa_state]
                            .insert(dfa::Alphabet::Char(c), next_dfa_state);
                    }
                }
            }
        }

        let mut end_states: HashSet<StateId> = HashSet::new();
        for (subset, &dfa_state) in &subset_to_dfa_state {
            for &end in nfa_end {
                if subset.contains(&end) {
                    end_states.insert(dfa_state);
                    break;
                }
            }
        }

        self.dfa.start = Some(dfa_start);
        self.dfa.end = Some(end_states);
    }

    fn generate_minimized_dfa(&mut self) {
        self.generate_dfa();

        let dfa_start = self.dfa.start.expect("Dfa should be generated already");
        let dfa_end = self
            .dfa
            .end
            .as_ref()
            .expect("Dfa should be generated already");

        let total_dfa_states = self.dfa.transitions.len();

        let mut reachable_states: HashSet<StateId> = HashSet::from([dfa_start]);
        let mut current_states: HashSet<StateId> = HashSet::from([dfa_start]);
        while !current_states.is_empty() {
            let mut next_states: HashSet<StateId> = HashSet::new();
            for &state in &current_states {
                for (&_alphabet, &next) in &self.dfa.transitions[state] {
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
                if dfa_end.contains(&curr) {
                    end_state_reachable = true;
                    break;
                }
                for (&_alphabet, &next) in &self.dfa.transitions[curr] {
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

            if dfa_end.contains(&i) {
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
                    let next = match self.dfa.transitions[curr].get(&dfa::Alphabet::Char(c)) {
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

        self.minimized_dfa.add_state();

        let largest_node = group_mapping
            .values()
            .max()
            .expect("there must be some nodes in minimized_dfa");

        while self.minimized_dfa.add_state() != *largest_node {}

        let mut minimized_dfa_start = 0;
        let mut minimized_dfa_end: HashSet<StateId> = HashSet::new();
        for (&dfa_state, &group) in &group_mapping {
            for c in u8::MIN..=u8::MAX {
                let c = c as char;
                if let Some(next_dfa_state) =
                    self.dfa.transitions[dfa_state].get(&dfa::Alphabet::Char(c))
                {
                    self.minimized_dfa.transitions[group].insert(
                        minimized_dfa::Alphabet::Char(c),
                        group_mapping[next_dfa_state],
                    );
                }
            }

            if dfa_state == dfa_start {
                minimized_dfa_start = group;
            }

            if dfa_end.contains(&dfa_state) {
                minimized_dfa_end.insert(group);
            }
        }

        self.minimized_dfa.start = Some(minimized_dfa_start);
        self.minimized_dfa.end = Some(minimized_dfa_end);
    }

    pub fn generate(&mut self) {
        self.generate_minimized_dfa();
    }

    pub fn check(&self, input: &str) -> Result<bool, String> {
        self.epsilon_nfa.as_ref().unwrap().check(input)?;
        self.nfa.check(input)?;
        self.dfa.check(input)?;
        self.minimized_dfa.check(input)
    }
}
