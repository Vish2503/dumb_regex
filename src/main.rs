use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    iter::Peekable,
    process::exit,
    str::Chars,
};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Alphabet {
    Char(char),
    Epsilon,
}
type StateId = usize;
type StatePair = (StateId, StateId);
type NFATransition = HashMap<Alphabet, HashSet<StateId>>;
type DFATransition = HashMap<Alphabet, StateId>;

#[derive(Debug)]
struct EpsilonNfa {
    transitions: Vec<NFATransition>,
    start: Option<StateId>,
    end: Option<StateId>,
}

impl EpsilonNfa {
    fn new() -> Self {
        Self {
            transitions: Vec::new(),
            start: None,
            end: None,
        }
    }

    fn add_state(&mut self) -> StateId {
        let state: StateId = self.transitions.len();
        self.transitions.push(NFATransition::new());
        state
    }

    fn add_epsilon_transition(&mut self, from: StateId, to: StateId) {
        self.transitions[from]
            .entry(Alphabet::Epsilon)
            .or_default()
            .insert(to);
    }

    fn add_transition(&mut self, from: StateId, c: char, to: StateId) {
        self.transitions[from]
            .entry(Alphabet::Char(c))
            .or_default()
            .insert(to);
    }

    fn add_transition_range<I: IntoIterator<Item = char>>(
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

    fn invert_transition(&mut self, from: StateId, to: StateId) {
        for c in u8::MIN..=u8::MAX {
            match self.transitions[from].remove(&Alphabet::Char(c as char)) {
                Some(_) => continue,
                None => {
                    self.transitions[from].insert(Alphabet::Char(c as char), HashSet::from([to]));
                }
            }
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

    fn make_deep_copy(&mut self, start: StateId, end: StateId) -> Result<StatePair, String> {
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

#[derive(Debug)]
struct Nfa {
    transitions: Vec<NFATransition>,
    start: Option<StateId>,
    end: Option<HashSet<StateId>>,
}

impl Nfa {
    fn new() -> Self {
        Self {
            transitions: Vec::new(),
            start: None,
            end: None,
        }
    }
}

#[derive(Debug)]
struct Dfa {
    transitions: Vec<DFATransition>,
    start: Option<StateId>,
    end: Option<HashSet<StateId>>,
}

impl Dfa {
    fn new() -> Self {
        Self {
            transitions: Vec::new(),
            start: None,
            end: None,
        }
    }

    fn add_state(&mut self) -> StateId {
        let state: StateId = self.transitions.len();
        self.transitions.push(DFATransition::new());
        state
    }
}

#[derive(Debug)]
struct MinimizedDfa {
    transitions: Vec<DFATransition>,
    start: Option<StateId>,
    end: Option<HashSet<StateId>>,
}

impl MinimizedDfa {
    fn new() -> Self {
        Self {
            transitions: Vec::new(),
            start: None,
            end: None,
        }
    }

    fn add_state(&mut self) -> StateId {
        let state: StateId = self.transitions.len();
        self.transitions.push(DFATransition::new());
        state
    }
}

struct RegularExpression<'a> {
    pattern_iter: Peekable<Chars<'a>>,
    epsilon_nfa: EpsilonNfa,
    nfa: Nfa,
    dfa: Dfa,
    minimized_dfa: MinimizedDfa,
}

impl<'a> RegularExpression<'a> {
    fn new(pattern: &'a str) -> Self {
        RegularExpression {
            pattern_iter: pattern.chars().peekable(),
            epsilon_nfa: EpsilonNfa::new(),
            nfa: Nfa::new(),
            dfa: Dfa::new(),
            minimized_dfa: MinimizedDfa::new(),
        }
    }

    fn parser_peek(&mut self) -> Option<char> {
        self.pattern_iter.peek().copied()
    }

    fn parser_match(&mut self, c: char) -> Result<char, String> {
        if let Some(value) = self.pattern_iter.next_if_eq(&c) {
            Ok(value)
        } else {
            Err(format!(
                "Expected {c} but found {value:?}",
                value = self.parser_peek()
            ))
        }
    }

    fn parser_match_one_of(&mut self, s: &str) -> Result<char, String> {
        if let Some(value) = self.pattern_iter.next_if(|&c| s.contains(c)) {
            Ok(value)
        } else {
            Err(format!(
                "Expected one of {s} but found {value:?}",
                value = self.parser_peek()
            ))
        }
    }

    fn parser_match_none_of(&mut self, s: &str) -> Result<char, String> {
        if let Some(value) = self.pattern_iter.next_if(|&c| !s.contains(c)) {
            Ok(value)
        } else {
            Err(format!(
                "Expected none of {s} but found {value:?}",
                value = self.parser_peek()
            ))
        }
    }

    fn parse_re(&mut self) -> Result<Option<StatePair>, String> {
        match self.parse_simple_re()? {
            Some(simple_re_res) => Ok(Some(self.parse_re_tail(simple_re_res)?)),
            None => Ok(None),
        }
    }

    fn parse_re_tail(&mut self, lvalue: StatePair) -> Result<StatePair, String> {
        match self.parser_peek() {
            Some('|') => {
                self.parser_match('|')?;

                let Some(simple_re_res) = self.parse_simple_re()? else {
                    return Err("Unexpectedly found no simple_re after `|`".to_string());
                };

                let start: StateId = self.epsilon_nfa.add_state();
                let end: StateId = self.epsilon_nfa.add_state();

                let (up_start, up_end) = lvalue;
                let (down_start, down_end) = simple_re_res;

                self.epsilon_nfa.add_epsilon_transition(start, up_start);
                self.epsilon_nfa.add_epsilon_transition(start, down_start);
                self.epsilon_nfa.add_epsilon_transition(up_end, end);
                self.epsilon_nfa.add_epsilon_transition(down_end, end);

                self.parse_re_tail((start, end))
            }
            _ => Ok(lvalue),
        }
    }

    fn parse_simple_re(&mut self) -> Result<Option<StatePair>, String> {
        match self.parse_basic_re()? {
            Some(simple_re_res) => Ok(Some(self.parse_simple_re_tail(simple_re_res)?)),
            None => Ok(None),
        }
    }

    fn parse_simple_re_tail(&mut self, lvalue: StatePair) -> Result<StatePair, String> {
        match self.parse_basic_re()? {
            Some(basic_re_res) => {
                let (left_start, left_end) = lvalue;
                let (right_start, right_end) = basic_re_res;

                self.epsilon_nfa
                    .add_epsilon_transition(left_end, right_start);

                self.parse_simple_re_tail((left_start, right_end))
            }
            None => Ok(lvalue),
        }
    }

    fn parse_basic_re(&mut self) -> Result<Option<StatePair>, String> {
        let Some(elementary_re_res) = self.parse_elementary_re()? else {
            return Ok(None);
        };

        match self.parser_peek() {
            Some('*') => {
                self.parser_match('*')?;

                let (elementary_re_start, elementary_re_end) = elementary_re_res;

                let start = self.epsilon_nfa.add_state();
                let end = self.epsilon_nfa.add_state();

                self.epsilon_nfa
                    .add_epsilon_transition(start, elementary_re_start);
                self.epsilon_nfa
                    .add_epsilon_transition(elementary_re_end, end);
                self.epsilon_nfa.add_epsilon_transition(start, end);
                self.epsilon_nfa
                    .add_epsilon_transition(elementary_re_end, elementary_re_start);

                Ok(Some((start, end)))
            }
            Some('+') => {
                self.parser_match('+')?;

                let (elementary_re_start, elementary_re_end) = elementary_re_res;

                let start = self.epsilon_nfa.add_state();
                let end = self.epsilon_nfa.add_state();

                self.epsilon_nfa
                    .add_epsilon_transition(start, elementary_re_start);
                self.epsilon_nfa
                    .add_epsilon_transition(elementary_re_end, end);
                self.epsilon_nfa
                    .add_epsilon_transition(elementary_re_end, elementary_re_start);

                Ok(Some((start, end)))
            }
            Some('?') => {
                self.parser_match('?')?;

                let (elementary_re_start, elementary_re_end) = elementary_re_res;

                let start = self.epsilon_nfa.add_state();
                let end = self.epsilon_nfa.add_state();

                self.epsilon_nfa
                    .add_epsilon_transition(start, elementary_re_start);
                self.epsilon_nfa
                    .add_epsilon_transition(elementary_re_end, end);
                self.epsilon_nfa.add_epsilon_transition(start, end);

                Ok(Some((start, end)))
            }
            Some('{') => {
                self.parser_match('{')?;

                let digits = "0123456789";

                let mut n: i32 = 0;
                let mut m: i32 = 0;
                while digits.contains(self.parser_peek().unwrap_or_default()) {
                    let c = self
                        .parser_match_one_of(digits)?
                        .to_digit(10)
                        .expect("c must be one of the digits");
                    n = n * 10 + c as i32;
                }

                match self.parser_peek() {
                    Some(',') => {
                        self.parser_match(',')?;

                        match self.parser_peek() {
                            Some(c) if digits.contains(c) => {
                                while digits.contains(self.parser_peek().unwrap_or_default()) {
                                    let c = self
                                        .parser_match_one_of(digits)?
                                        .to_digit(10)
                                        .expect("c must be one of the digits");
                                    m = m * 10 + c as i32;
                                }
                            }
                            _ => {
                                m = -1;
                            }
                        }
                    }
                    _ => {
                        m = n;
                    }
                }

                self.parser_match('}')?;

                if m != -1 && m < n {
                    return Err("Out of order range found in the pattern".to_string());
                }

                let start = self.epsilon_nfa.add_state();
                let end = self.epsilon_nfa.add_state();

                let (elementary_re_start, elementary_re_end) = elementary_re_res;

                if n == 0 {
                    self.epsilon_nfa.add_epsilon_transition(start, end);
                }

                let mut repeated_elementary_re_start: Option<StateId> = None;
                let mut repeated_elementary_re_end: Option<StateId> = None;

                for _ in 1..=n {
                    let (elementary_re_copy_start, elementary_re_copy_end) = self
                        .epsilon_nfa
                        .make_deep_copy(elementary_re_start, elementary_re_end)?;

                    if repeated_elementary_re_start.is_none()
                        && repeated_elementary_re_end.is_none()
                    {
                        repeated_elementary_re_start = Some(elementary_re_copy_start);
                        repeated_elementary_re_end = Some(elementary_re_copy_end);
                    } else {
                        self.epsilon_nfa.add_epsilon_transition(
                            repeated_elementary_re_end.unwrap(),
                            elementary_re_copy_start,
                        );
                        repeated_elementary_re_end = Some(elementary_re_copy_end);
                    }
                }

                if m == -1 {
                    let (elementary_re_copy_start, elementary_re_copy_end) = self
                        .epsilon_nfa
                        .make_deep_copy(elementary_re_start, elementary_re_end)?;

                    let new_elementary_re_copy_start = self.epsilon_nfa.add_state();
                    let new_elementary_re_copy_end = self.epsilon_nfa.add_state();

                    self.epsilon_nfa.add_epsilon_transition(
                        new_elementary_re_copy_start,
                        elementary_re_copy_start,
                    );
                    self.epsilon_nfa
                        .add_epsilon_transition(elementary_re_copy_end, new_elementary_re_copy_end);

                    self.epsilon_nfa
                        .add_epsilon_transition(elementary_re_copy_end, elementary_re_copy_start);
                    self.epsilon_nfa.add_epsilon_transition(
                        new_elementary_re_copy_start,
                        new_elementary_re_copy_end,
                    );

                    if repeated_elementary_re_start.is_none()
                        && repeated_elementary_re_end.is_none()
                    {
                        repeated_elementary_re_start = Some(new_elementary_re_copy_start);
                        repeated_elementary_re_end = Some(new_elementary_re_copy_end);
                    } else {
                        self.epsilon_nfa.add_epsilon_transition(
                            repeated_elementary_re_end.unwrap(),
                            new_elementary_re_copy_start,
                        );
                        repeated_elementary_re_end = Some(new_elementary_re_copy_end);
                    }
                }

                for _ in n + 1..=m {
                    let (elementary_re_copy_start, elementary_re_copy_end) = self
                        .epsilon_nfa
                        .make_deep_copy(elementary_re_start, elementary_re_end)?;

                    let new_elementary_re_copy_start = self.epsilon_nfa.add_state();
                    let new_elementary_re_copy_end = self.epsilon_nfa.add_state();

                    self.epsilon_nfa.add_epsilon_transition(
                        new_elementary_re_copy_start,
                        elementary_re_copy_start,
                    );
                    self.epsilon_nfa
                        .add_epsilon_transition(elementary_re_copy_end, new_elementary_re_copy_end);

                    self.epsilon_nfa.add_epsilon_transition(
                        new_elementary_re_copy_start,
                        new_elementary_re_copy_end,
                    );

                    if repeated_elementary_re_start.is_none()
                        && repeated_elementary_re_end.is_none()
                    {
                        repeated_elementary_re_start = Some(new_elementary_re_copy_start);
                        repeated_elementary_re_end = Some(new_elementary_re_copy_end);
                    } else {
                        self.epsilon_nfa.add_epsilon_transition(
                            repeated_elementary_re_end.unwrap(),
                            new_elementary_re_copy_start,
                        );
                        repeated_elementary_re_end = Some(new_elementary_re_copy_end);
                    }
                }

                if let (Some(repeated_elementary_re_start), Some(repeated_elementary_re_end)) =
                    (repeated_elementary_re_start, repeated_elementary_re_end)
                {
                    self.epsilon_nfa
                        .add_epsilon_transition(start, repeated_elementary_re_start);
                    self.epsilon_nfa
                        .add_epsilon_transition(repeated_elementary_re_end, end);
                }

                Ok(Some((start, end)))
            }
            _ => Ok(Some(elementary_re_res)),
        }
    }

    fn parse_elementary_re(&mut self) -> Result<Option<StatePair>, String> {
        if let Some(group_res) = self.parse_group()? {
            return Ok(Some(group_res));
        }

        if let Some(any_res) = self.parse_any()? {
            return Ok(Some(any_res));
        }

        if let Some(char_res) = self.parse_char()? {
            return Ok(Some(char_res));
        }

        if let Some(set_res) = self.parse_set()? {
            return Ok(Some(set_res));
        }

        Ok(None)
    }

    fn parse_group(&mut self) -> Result<Option<StatePair>, String> {
        match self.parser_peek() {
            Some('(') => {
                self.parser_match('(')?;
                let re_res = self.parse_re()?;
                self.parser_match(')')?;
                Ok(re_res)
            }
            _ => Ok(None),
        }
    }

    fn parse_any(&mut self) -> Result<Option<StatePair>, String> {
        match self.parser_peek() {
            Some('.') => {
                self.parser_match('.')?;

                let start = self.epsilon_nfa.add_state();
                let end = self.epsilon_nfa.add_state();

                for c in u8::MIN..=u8::MAX {
                    self.epsilon_nfa.add_transition(start, c as char, end);
                }

                Ok(Some((start, end)))
            }
            _ => Ok(None),
        }
    }

    fn parse_char(&mut self) -> Result<Option<StatePair>, String> {
        let meta_characters = "[]\\.^$*+?{}|()";
        let possible_escape_characters = "[]\\.^$*+?{}|()wWsSdDnrt";
        let white_space = "\t\n\r ";
        match self.parser_peek() {
            Some('\\') => {
                self.parser_match('\\')?;
                match self.parser_match_one_of(possible_escape_characters)? {
                    c if meta_characters.contains(c) => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();
                        self.epsilon_nfa.add_transition(start, c, end);

                        Ok(Some((start, end)))
                    }
                    'w' => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();

                        self.epsilon_nfa.add_transition_range(
                            start,
                            ('a'..='z')
                                .chain('A'..='Z')
                                .chain('0'..='9')
                                .chain('_'..='_'),
                            end,
                        );

                        Ok(Some((start, end)))
                    }
                    'W' => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();

                        self.epsilon_nfa.add_transition_range(
                            start,
                            (u8::MIN..=u8::MAX)
                                .map(|c| c as char)
                                .filter(|c| !c.is_alphanumeric() && *c != '_'),
                            end,
                        );

                        Ok(Some((start, end)))
                    }
                    's' => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();

                        self.epsilon_nfa
                            .add_transition_range(start, white_space.chars(), end);

                        Ok(Some((start, end)))
                    }
                    'S' => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();

                        self.epsilon_nfa.add_transition_range(
                            start,
                            (u8::MIN..=u8::MAX)
                                .map(|c| c as char)
                                .filter(|&c| !white_space.contains(c)),
                            end,
                        );

                        Ok(Some((start, end)))
                    }
                    'd' => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();

                        self.epsilon_nfa.add_transition_range(start, '0'..='9', end);

                        Ok(Some((start, end)))
                    }
                    'D' => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();

                        self.epsilon_nfa.add_transition_range(
                            start,
                            (u8::MIN..=u8::MAX)
                                .map(|c| c as char)
                                .filter(|c| !c.is_ascii_digit()),
                            end,
                        );

                        Ok(Some((start, end)))
                    }
                    'n' => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();
                        self.epsilon_nfa.add_transition(start, '\n', end);

                        Ok(Some((start, end)))
                    }
                    'r' => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();
                        self.epsilon_nfa.add_transition(start, '\r', end);

                        Ok(Some((start, end)))
                    }
                    't' => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();
                        self.epsilon_nfa.add_transition(start, '\t', end);

                        Ok(Some((start, end)))
                    }
                    _ => Err("Unexpected behaviour in parse_char()".to_string()),
                }
            }
            Some(c) => {
                if meta_characters.contains(c) {
                    return Ok(None);
                }

                let c = self.parser_match_none_of(meta_characters)?;

                let start = self.epsilon_nfa.add_state();
                let end = self.epsilon_nfa.add_state();
                self.epsilon_nfa.add_transition(start, c, end);

                Ok(Some((start, end)))
            }
            None => Ok(None),
        }
    }

    fn parse_set(&mut self) -> Result<Option<StatePair>, String> {
        match self.parser_peek() {
            Some('[') => {
                self.parser_match('[')?;

                let negate = if self.parser_peek() == Some('^') {
                    self.parser_match('^')?;
                    true
                } else {
                    false
                };

                let Some((start, end)) = self.parse_set_items()? else {
                    return Err("Empty character set found in the pattern.".to_string());
                };

                self.parser_match(']')?;

                if negate {
                    self.epsilon_nfa.invert_transition(start, end);
                }

                Ok(Some((start, end)))
            }
            _ => Ok(None),
        }
    }

    fn parse_set_items(&mut self) -> Result<Option<StatePair>, String> {
        let Some(set_item_res) = self.parse_set_item()? else {
            return Ok(None);
        };

        let Some(set_items_res) = self.parse_set_items()? else {
            return Ok(Some(set_item_res));
        };

        let (old_start, _old_end) = set_items_res;
        let (start, end) = set_item_res;

        let [old_map, map] = self
            .epsilon_nfa
            .transitions
            .get_disjoint_mut([old_start, start])
            .expect("old_start and start must always be non-overlapping indices inside the transitions vector");

        for &k in old_map.keys() {
            map.entry(k).or_default().insert(end);
        }

        Ok(Some((start, end)))
    }

    fn parse_set_item(&mut self) -> Result<Option<StatePair>, String> {
        match self.parse_set_char()? {
            Some(char_res) => Ok(Some(self.parse_range(char_res)?)),
            _ => Ok(None),
        }
    }

    fn parse_range(&mut self, lvalue: StatePair) -> Result<StatePair, String> {
        let Some('-') = self.parser_peek() else {
            return Ok(lvalue);
        };

        self.parser_match('-')?;

        let (start, end) = lvalue;

        match self.parse_set_char()? {
            None => {
                self.epsilon_nfa.add_transition(start, '-', end);
                Ok((start, end))
            }
            Some(char_res) => {
                let range_start = match self.epsilon_nfa.transitions[start].keys().next() {
                    Some(Alphabet::Char(c)) => *c as u8,
                    _ => {
                        return Err("Unexpected behaviour in parse_range()".to_string());
                    }
                };
                let range_end = match self.epsilon_nfa.transitions[char_res.0].keys().next() {
                    Some(Alphabet::Char(c)) => *c as u8,
                    _ => {
                        return Err("Unexpected behaviour in parse_range()".to_string());
                    }
                };

                if range_start > range_end {
                    self.epsilon_nfa
                        .add_transition(start, range_start as char, end);
                    self.epsilon_nfa.add_transition(start, '-', end);
                    self.epsilon_nfa
                        .add_transition(start, range_end as char, end);
                } else {
                    self.epsilon_nfa.add_transition_range(
                        start,
                        (range_start..=range_end).map(|c| c as char),
                        end,
                    );
                }

                Ok((start, end))
            }
        }
    }

    fn parse_set_char(&mut self) -> Result<Option<StatePair>, String> {
        let meta_characters = "[]\\";
        let possible_escape_characters = "[]\\nrt";
        match self.parser_peek() {
            Some('\\') => {
                self.parser_match('\\')?;
                match self.parser_match_one_of(possible_escape_characters)? {
                    c if meta_characters.contains(c) => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();
                        self.epsilon_nfa.add_transition(start, c, end);

                        Ok(Some((start, end)))
                    }
                    'n' => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();
                        self.epsilon_nfa.add_transition(start, '\n', end);

                        Ok(Some((start, end)))
                    }
                    'r' => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();
                        self.epsilon_nfa.add_transition(start, '\r', end);

                        Ok(Some((start, end)))
                    }
                    't' => {
                        let start = self.epsilon_nfa.add_state();
                        let end = self.epsilon_nfa.add_state();
                        self.epsilon_nfa.add_transition(start, '\t', end);

                        Ok(Some((start, end)))
                    }
                    _ => Err("Unexpected behaviour in parse_char()".to_string()),
                }
            }
            Some(c) => {
                if meta_characters.contains(c) {
                    return Ok(None);
                }

                let c = self.parser_match_none_of(meta_characters)?;

                let start = self.epsilon_nfa.add_state();
                let end = self.epsilon_nfa.add_state();
                self.epsilon_nfa.add_transition(start, c, end);

                Ok(Some((start, end)))
            }
            None => Ok(None),
        }
    }

    fn generate_epsilon_nfa(&mut self) {
        match self.parse_re() {
            Ok(Some((start, end))) => {
                self.epsilon_nfa.start = Some(start);
                self.epsilon_nfa.end = Some(end);
            }
            Ok(None) => {
                eprintln!("Error generating epsilon nfa: parse_re() returned None");
                exit(1);
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
            .start
            .expect("Epsilon Nfa should be generated already");
        let epsilon_nfa_end = self
            .epsilon_nfa
            .end
            .expect("Epsilon Nfa should be generated already");

        let n = self.epsilon_nfa.transitions.len();
        self.nfa.transitions.resize(n, Default::default());

        let start = epsilon_nfa_start;
        let mut end: HashSet<StateId> = HashSet::new();

        let mut state_epsilon_closure: Vec<HashSet<StateId>> = Vec::new();
        state_epsilon_closure.resize(n, Default::default());
        for (curr, curr_epsilon_closure) in state_epsilon_closure.iter_mut().enumerate() {
            self.epsilon_nfa.epsilon_closure(curr, curr_epsilon_closure);

            if curr_epsilon_closure.contains(&epsilon_nfa_end) {
                end.insert(curr);
            }
        }

        for curr in 0..n {
            for &epsilon_state in &state_epsilon_closure[curr] {
                for (&alphabet, next_states) in &self.epsilon_nfa.transitions[epsilon_state] {
                    if let Alphabet::Char(_) = alphabet {
                        for &next in next_states {
                            self.nfa.transitions[curr]
                                .entry(alphabet)
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

            let mut current_transitions: HashMap<Alphabet, BTreeSet<StateId>> = HashMap::new();
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
                self.dfa.transitions[curr_dfa_state].insert(alphabet, next_dfa_state);
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
                    let next = match self.dfa.transitions[curr].get(&Alphabet::Char(c)) {
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
                    self.dfa.transitions[dfa_state].get(&Alphabet::Char(c))
                {
                    self.minimized_dfa.transitions[group]
                        .insert(Alphabet::Char(c), group_mapping[next_dfa_state]);
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

    fn generate(&mut self) {
        self.generate_minimized_dfa();
    }

    fn check_epsilon_nfa(&self, input: &str) -> Result<bool, String> {
        let Some(start) = self.epsilon_nfa.start else {
            return Err("Epsilon Nfa start has not been initialized yet".to_string());
        };
        let Some(end) = self.epsilon_nfa.end else {
            return Err("Epsilon Nfa end has not been initialized yet".to_string());
        };

        let mut epsilon_closure_start: HashSet<StateId> = HashSet::new();
        self.epsilon_nfa
            .epsilon_closure(start, &mut epsilon_closure_start);

        let mut current_states: HashSet<StateId> = HashSet::new();
        current_states.extend(epsilon_closure_start);
        for c in input.chars() {
            let mut next_states: HashSet<StateId> = HashSet::new();
            for &curr in &current_states {
                if let Some(adj) = self.epsilon_nfa.transitions[curr].get(&Alphabet::Char(c)) {
                    for &next in adj {
                        let mut epsilon_closure_next: HashSet<StateId> = HashSet::new();
                        self.epsilon_nfa
                            .epsilon_closure(next, &mut epsilon_closure_next);
                        next_states.extend(epsilon_closure_next);
                    }
                }
            }
            current_states = next_states;
        }

        Ok(current_states.contains(&end))
    }

    fn check_nfa(&self, input: &str) -> Result<bool, String> {
        let Some(start) = self.nfa.start else {
            return Err("Nfa start has not been initialized yet".to_string());
        };
        let Some(ref end_states) = self.nfa.end else {
            return Err("Nfa end has not been initialized yet".to_string());
        };

        let mut current_states: HashSet<StateId> = HashSet::new();
        current_states.insert(start);
        for c in input.chars() {
            let mut next_states: HashSet<StateId> = HashSet::new();
            for &curr in &current_states {
                if let Some(adj) = self.nfa.transitions[curr].get(&Alphabet::Char(c)) {
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

    fn check_dfa(&self, input: &str) -> Result<bool, String> {
        let Some(start) = self.dfa.start else {
            return Err("Nfa start has not been initialized yet".to_string());
        };
        let Some(ref end_states) = self.dfa.end else {
            return Err("Nfa end has not been initialized yet".to_string());
        };

        let mut curr: StateId = start;
        for c in input.chars() {
            if let Some(&next) = self.dfa.transitions[curr].get(&Alphabet::Char(c)) {
                curr = next;
            } else {
                return Ok(false);
            }
        }

        Ok(end_states.contains(&curr))
    }

    fn check_minimized_dfa(&self, input: &str) -> Result<bool, String> {
        let Some(start) = self.minimized_dfa.start else {
            return Err("Nfa start has not been initialized yet".to_string());
        };
        let Some(ref end_states) = self.minimized_dfa.end else {
            return Err("Nfa end has not been initialized yet".to_string());
        };

        let mut curr: StateId = start;
        for c in input.chars() {
            if let Some(&next) = self.minimized_dfa.transitions[curr].get(&Alphabet::Char(c)) {
                curr = next;
            } else {
                return Ok(false);
            }
        }

        Ok(end_states.contains(&curr))
    }

    fn check(self, input: &str) -> Result<bool, String> {
        self.check_epsilon_nfa(input)?;
        self.check_nfa(input)?;
        self.check_dfa(input)?;
        self.check_minimized_dfa(input)
    }
}

fn main() {
    let pattern: &str = "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?";
    let mut re: RegularExpression = RegularExpression::new(pattern);
    re.generate();
    match re.check("1") {
        Ok(res) => {
            if res {
                println!("matches");
            } else {
                println!("does not match");
            }
        }
        Err(e) => {
            eprintln!("Error while checking: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RegularExpression;

    #[test]
    fn basic_test_1() {
        let mut re: RegularExpression = RegularExpression::new("a");
        re.generate();
        assert_eq!(re.check("a"), Ok(true));
    }

    #[test]
    fn basic_test_2() {
        let mut re: RegularExpression = RegularExpression::new("a");
        re.generate();
        assert_eq!(re.check("b"), Ok(false));
    }

    #[test]
    fn basic_test_3() {
        let mut re: RegularExpression = RegularExpression::new("a");
        re.generate();
        assert_eq!(re.check("ab"), Ok(false));
    }

    #[test]
    fn star_test_1() {
        let mut re: RegularExpression = RegularExpression::new("a*");
        re.generate();
        assert_eq!(re.check("aaaaaaaaaaa"), Ok(true));
    }

    #[test]
    fn star_test_2() {
        let mut re: RegularExpression = RegularExpression::new("a*");
        re.generate();
        assert_eq!(re.check("aaaaaaaaaabaaaaaa"), Ok(false));
    }

    #[test]
    fn union_test_1() {
        let mut re: RegularExpression = RegularExpression::new("a|b|c");
        re.generate();
        assert_eq!(re.check("a"), Ok(true));
    }

    #[test]
    fn union_test_2() {
        let mut re: RegularExpression = RegularExpression::new("a|b|c");
        re.generate();
        assert_eq!(re.check("b"), Ok(true));
    }

    #[test]
    fn union_test_3() {
        let mut re: RegularExpression = RegularExpression::new("a|b|c");
        re.generate();
        assert_eq!(re.check("d"), Ok(false));
    }

    #[test]
    fn char_set_test_1() {
        let mut re: RegularExpression = RegularExpression::new("[hc]at");
        re.generate();
        assert_eq!(re.check("hat"), Ok(true));
    }

    #[test]
    fn char_set_test_2() {
        let mut re: RegularExpression = RegularExpression::new("[hc]at");
        re.generate();
        assert_eq!(re.check("cat"), Ok(true));
    }

    #[test]
    fn char_set_test_3() {
        let mut re: RegularExpression = RegularExpression::new("[hc]at");
        re.generate();
        assert_eq!(re.check("mat"), Ok(false));
    }

    #[test]
    fn any_test_1() {
        let mut re: RegularExpression = RegularExpression::new(".at");
        re.generate();
        assert_eq!(re.check("hat"), Ok(true));
    }

    #[test]
    fn any_test_2() {
        let mut re: RegularExpression = RegularExpression::new(".at");
        re.generate();
        assert_eq!(re.check("cat"), Ok(true));
    }

    #[test]
    fn any_test_3() {
        let mut re: RegularExpression = RegularExpression::new(".at");
        re.generate();
        assert_eq!(re.check("mat"), Ok(true));
    }

    #[test]
    fn any_test_4() {
        let mut re: RegularExpression = RegularExpression::new(".at");
        re.generate();
        assert_eq!(re.check("pat"), Ok(true));
    }

    #[test]
    fn group_test_1() {
        let mut re: RegularExpression = RegularExpression::new("([hc]at)?[mp]at");
        re.generate();
        assert_eq!(re.check("mat"), Ok(true));
    }

    #[test]
    fn group_test_2() {
        let mut re: RegularExpression = RegularExpression::new("([hc]at)?[mp]at");
        re.generate();
        assert_eq!(re.check("hat"), Ok(false));
    }

    #[test]
    fn group_test_3() {
        let mut re: RegularExpression = RegularExpression::new("([hc]at)?[mp]at");
        re.generate();
        assert_eq!(re.check("pat"), Ok(true));
    }

    #[test]
    fn group_test_4() {
        let mut re: RegularExpression = RegularExpression::new("([hc]at)?[mp]at");
        re.generate();
        assert_eq!(re.check("catmat"), Ok(true));
    }

    #[test]
    fn set_range_test_1() {
        let mut re: RegularExpression = RegularExpression::new("[a-zA-Z0-9]");
        re.generate();
        assert_eq!(re.check("5"), Ok(true));
    }

    #[test]
    fn set_range_test_2() {
        let mut re: RegularExpression = RegularExpression::new("[a-zA-Z0-9]");
        re.generate();
        assert_eq!(re.check("G"), Ok(true));
    }

    #[test]
    fn set_range_test_4() {
        let mut re: RegularExpression = RegularExpression::new("[a-zA-Z0-9]");
        re.generate();
        assert_eq!(re.check("@"), Ok(false));
    }

    #[test]
    fn special_char_test_1() {
        let mut re: RegularExpression = RegularExpression::new("\\w*");
        re.generate();
        assert_eq!(re.check("0123"), Ok(true));
    }
    #[test]
    fn special_char_test_2() {
        let mut re: RegularExpression = RegularExpression::new("\\w*");
        re.generate();
        assert_eq!(re.check("ZYX"), Ok(true));
    }

    #[test]
    fn special_char_test_3() {
        let mut re: RegularExpression = RegularExpression::new("\\w*");
        re.generate();
        assert_eq!(re.check("abcd"), Ok(true));
    }

    #[test]
    fn special_char_test_4() {
        let mut re: RegularExpression = RegularExpression::new("\\w*");
        re.generate();
        assert_eq!(re.check("abcdef_ABCDEF___01234"), Ok(true));
    }

    #[test]
    fn special_char_test_5() {
        let mut re: RegularExpression = RegularExpression::new("\\w*");
        re.generate();
        assert_eq!(re.check("0+1-2"), Ok(false));
    }

    #[test]
    fn numeral_test_1() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("1"), Ok(true));
    }

    #[test]
    fn numeral_test_2() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("1000000"), Ok(true));
    }

    #[test]
    fn numeral_test_3() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("-1"), Ok(true));
    }

    #[test]
    fn numeral_test_4() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("1e9"), Ok(true));
    }

    #[test]
    fn numeral_test_5() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("1e-5"), Ok(true));
    }

    #[test]
    fn numeral_test_6() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("1E-5"), Ok(true));
    }

    #[test]
    fn numeral_test_7() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("1e-12233342"), Ok(true));
    }

    #[test]
    fn numeral_test_8() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("3.1415926535"), Ok(true));
    }

    #[test]
    fn numeral_test_9() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("237429342e24801"), Ok(true));
    }

    #[test]
    fn numeral_test_10() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("6.022e+23"), Ok(true));
    }

    #[test]
    fn numeral_test_11() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("e+23"), Ok(false));
    }

    #[test]
    fn numeral_test_12() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("abcd"), Ok(false));
    }

    #[test]
    fn numeral_test_13() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("abcd123"), Ok(false));
    }

    #[test]
    fn numeral_test_14() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate();
        assert_eq!(re.check("123abcd"), Ok(false));
    }

    #[test]
    fn ab_test_1() {
        let mut re: RegularExpression = RegularExpression::new("(a|b)*abb(a|b)*");
        re.generate();
        assert_eq!(re.check("aaaabbbbbb"), Ok(true));
    }

    #[test]
    fn easy_test_1() {
        let mut re: RegularExpression = RegularExpression::new("(a*|b*)*");
        re.generate();
        assert_eq!(re.check(""), Ok(true));
    }

    #[test]
    fn counted_repetition_test_1() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){0}");
        re.generate();
        assert_eq!(re.check(""), Ok(true));
    }

    #[test]
    fn counted_repetition_test_2() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){0,0}");
        re.generate();
        assert_eq!(re.check(""), Ok(true));
    }

    #[test]
    fn counted_repetition_test_3() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){0,0}");
        re.generate();
        assert_eq!(re.check("a"), Ok(false));
    }

    #[test]
    fn counted_repetition_test_4() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){0,1}");
        re.generate();
        assert_eq!(re.check(""), Ok(true));
    }

    #[test]
    fn counted_repetition_test_5() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){0,1}");
        re.generate();
        assert_eq!(re.check("a"), Ok(true));
    }

    #[test]
    fn counted_repetition_test_6() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){0,1}");
        re.generate();
        assert_eq!(re.check("ab"), Ok(false));
    }

    #[test]
    fn counted_repetition_test_7() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,4}");
        re.generate();
        assert_eq!(re.check(""), Ok(false));
    }

    #[test]
    fn counted_repetition_test_8() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,4}");
        re.generate();
        assert_eq!(re.check("a"), Ok(false));
    }

    #[test]
    fn counted_repetition_test_9() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,4}");
        re.generate();
        assert_eq!(re.check("ba"), Ok(true));
    }

    #[test]
    fn counted_repetition_test_10() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,4}");
        re.generate();
        assert_eq!(re.check("aba"), Ok(true));
    }

    #[test]
    fn counted_repetition_test_11() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,4}");
        re.generate();
        assert_eq!(re.check("aaba"), Ok(true));
    }

    #[test]
    fn counted_repetition_test_12() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,4}");
        re.generate();
        assert_eq!(re.check("abbaa"), Ok(false));
    }

    #[test]
    fn counted_repetition_test_13() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,}");
        re.generate();
        assert_eq!(re.check("aaaaaaaaaaaa"), Ok(true));
    }

    #[test]
    fn counted_repetition_test_14() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2}");
        re.generate();
        assert_eq!(re.check("a"), Ok(false));
    }

    #[test]
    fn counted_repetition_test_15() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2}");
        re.generate();
        assert_eq!(re.check("abb"), Ok(false));
    }

    #[test]
    fn counted_repetition_test_16() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){10,10}");
        re.generate();
        assert_eq!(re.check("abaaa"), Ok(false));
    }

    #[test]
    fn email_test_1() {
        let mut re: RegularExpression =
            RegularExpression::new("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}");
        re.generate();
        assert_eq!(re.check("john.smith@example.com"), Ok(true));
    }
}
