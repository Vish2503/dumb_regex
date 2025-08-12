use std::{
    collections::{HashMap, HashSet},
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

#[derive(Debug)]
struct EpsilonNFA {
    transitions: Vec<NFATransition>,
    start: Option<StateId>,
    end: Option<StateId>,
}

impl EpsilonNFA {
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
            .or_insert(HashSet::new())
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

    fn make_deep_copy(&mut self, start: StateId, end: StateId) -> StatePair {
        let mut mappings: HashMap<StateId, StateId> = HashMap::new();
        mappings.insert(start, self.add_state());

        let mut stack: Vec<StateId> = Vec::new();
        stack.push(start);
        while let Some(curr) = stack.pop() {
            for c in u8::MIN..=u8::MAX {
                let c = c as char;
                let transitions = &self.transitions[curr];
                if let Some(next_states) = transitions.get(&Alphabet::Char(c)) {
                    for next in next_states {
                        if !mappings.contains_key(&next) {
                            mappings.insert(*next, self.add_state());
                            stack.push(*next);
                        }
                    }
                }
            }
        }

        (mappings[&start], mappings[&end])
    }
}

struct RegularExpression<'a> {
    pattern_iter: Peekable<Chars<'a>>,
    epsilon_nfa: EpsilonNFA,
}

impl<'a> RegularExpression<'a> {
    fn new(pattern: &'a str) -> Self {
        RegularExpression {
            pattern_iter: pattern.chars().peekable(),
            epsilon_nfa: EpsilonNFA::new(),
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
            Some(value) if value == '|' => {
                self.parser_match('|')?;

                let simple_re_res: Option<StatePair> = self.parse_simple_re()?;

                let start: StateId = self.epsilon_nfa.add_state();
                let end: StateId = self.epsilon_nfa.add_state();

                let (up_start, up_end) = lvalue;
                let (down_start, down_end) =
                    simple_re_res.expect("parse_simple_re() returned None inside parse_re_tail");

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
                    let c = self.parser_match_one_of(digits)?;
                    n = n * 10
                        + c.to_digit(10)
                            .expect("Unexpected behaviour in parse_basic_re()")
                            as i32;
                }

                match self.parser_peek() {
                    Some(',') => {
                        self.parser_match(',')?;

                        match self.parser_peek() {
                            Some(c) if digits.contains(c) => {
                                m = 0;
                                while digits.contains(self.parser_peek().unwrap_or_default()) {
                                    let c = self.parser_match_one_of(digits)?;
                                    m = m * 10
                                        + c.to_digit(10)
                                            .expect("Unexpected behaviour in parse_basic_re()")
                                            as i32;
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

                if n == 0 {
                    self.epsilon_nfa.add_epsilon_transition(start, end);
                }

                todo!()
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
                                .filter(|c| !('0'..='9').contains(c)),
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
            .expect("Unexpected error in parse_set_items");

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

    fn check(self, input: &str) -> Result<bool, String> {
        let Some(start) = self.epsilon_nfa.start else {
            return Err("Epsilon NFA start has not been initialized yet".to_string());
        };
        let Some(end) = self.epsilon_nfa.end else {
            return Err("Epsilon NFA end has not been initialized yet".to_string());
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
}

fn main() {
    let pattern: &str = "\\w";
    let mut re: RegularExpression = RegularExpression::new(pattern);
    re.generate_epsilon_nfa();
    match re.check("w") {
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
        re.generate_epsilon_nfa();
        assert_eq!(re.check("a"), Ok(true));
    }

    #[test]
    fn basic_test_2() {
        let mut re: RegularExpression = RegularExpression::new("a");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("b"), Ok(false));
    }

    #[test]
    fn basic_test_3() {
        let mut re: RegularExpression = RegularExpression::new("a");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("ab"), Ok(false));
    }

    #[test]
    fn star_test_1() {
        let mut re: RegularExpression = RegularExpression::new("a*");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("aaaaaaaaaaa"), Ok(true));
    }

    #[test]
    fn star_test_2() {
        let mut re: RegularExpression = RegularExpression::new("a*");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("aaaaaaaaaabaaaaaa"), Ok(false));
    }

    #[test]
    fn union_test_1() {
        let mut re: RegularExpression = RegularExpression::new("a|b|c");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("a"), Ok(true));
    }

    #[test]
    fn union_test_2() {
        let mut re: RegularExpression = RegularExpression::new("a|b|c");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("b"), Ok(true));
    }

    #[test]
    fn union_test_3() {
        let mut re: RegularExpression = RegularExpression::new("a|b|c");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("d"), Ok(false));
    }

    #[test]
    fn char_set_test_1() {
        let mut re: RegularExpression = RegularExpression::new("[hc]at");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("hat"), Ok(true));
    }

    #[test]
    fn char_set_test_2() {
        let mut re: RegularExpression = RegularExpression::new("[hc]at");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("cat"), Ok(true));
    }

    #[test]
    fn char_set_test_3() {
        let mut re: RegularExpression = RegularExpression::new("[hc]at");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("mat"), Ok(false));
    }

    #[test]
    fn any_test_1() {
        let mut re: RegularExpression = RegularExpression::new(".at");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("hat"), Ok(true));
    }

    #[test]
    fn any_test_2() {
        let mut re: RegularExpression = RegularExpression::new(".at");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("cat"), Ok(true));
    }

    #[test]
    fn any_test_3() {
        let mut re: RegularExpression = RegularExpression::new(".at");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("mat"), Ok(true));
    }

    #[test]
    fn any_test_4() {
        let mut re: RegularExpression = RegularExpression::new(".at");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("pat"), Ok(true));
    }

    #[test]
    fn group_test_1() {
        let mut re: RegularExpression = RegularExpression::new("([hc]at)?[mp]at");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("mat"), Ok(true));
    }

    #[test]
    fn group_test_2() {
        let mut re: RegularExpression = RegularExpression::new("([hc]at)?[mp]at");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("hat"), Ok(false));
    }

    #[test]
    fn group_test_3() {
        let mut re: RegularExpression = RegularExpression::new("([hc]at)?[mp]at");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("pat"), Ok(true));
    }

    #[test]
    fn group_test_4() {
        let mut re: RegularExpression = RegularExpression::new("([hc]at)?[mp]at");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("catmat"), Ok(true));
    }

    #[test]
    fn set_range_test_1() {
        let mut re: RegularExpression = RegularExpression::new("[a-zA-Z0-9]");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("5"), Ok(true));
    }

    #[test]
    fn set_range_test_2() {
        let mut re: RegularExpression = RegularExpression::new("[a-zA-Z0-9]");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("G"), Ok(true));
    }

    #[test]
    fn set_range_test_4() {
        let mut re: RegularExpression = RegularExpression::new("[a-zA-Z0-9]");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("@"), Ok(false));
    }

    #[test]
    fn special_char_test_1() {
        let mut re: RegularExpression = RegularExpression::new("\\w*");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("0123"), Ok(true));
    }
    #[test]
    fn special_char_test_2() {
        let mut re: RegularExpression = RegularExpression::new("\\w*");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("ZYX"), Ok(true));
    }

    #[test]
    fn special_char_test_3() {
        let mut re: RegularExpression = RegularExpression::new("\\w*");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("abcd"), Ok(true));
    }

    #[test]
    fn special_char_test_4() {
        let mut re: RegularExpression = RegularExpression::new("\\w*");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("abcdef_ABCDEF___01234"), Ok(true));
    }

    #[test]
    fn special_char_test_5() {
        let mut re: RegularExpression = RegularExpression::new("\\w*");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("0+1-2"), Ok(false));
    }

    #[test]
    fn numeral_test_1() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("1"), Ok(true));
    }

    #[test]
    fn numeral_test_2() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("1000000"), Ok(true));
    }

    #[test]
    fn numeral_test_3() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("-1"), Ok(true));
    }

    #[test]
    fn numeral_test_4() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("1e9"), Ok(true));
    }

    #[test]
    fn numeral_test_5() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("1e-5"), Ok(true));
    }

    #[test]
    fn numeral_test_6() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("1E-5"), Ok(true));
    }

    #[test]
    fn numeral_test_7() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("1e-12233342"), Ok(true));
    }

    #[test]
    fn numeral_test_8() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("3.1415926535"), Ok(true));
    }

    #[test]
    fn numeral_test_9() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("237429342e24801"), Ok(true));
    }

    #[test]
    fn numeral_test_10() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("6.022e+23"), Ok(true));
    }

    #[test]
    fn numeral_test_11() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("e+23"), Ok(false));
    }

    #[test]
    fn numeral_test_12() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("abcd"), Ok(false));
    }

    #[test]
    fn numeral_test_13() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("abcd123"), Ok(false));
    }

    #[test]
    fn numeral_test_14() {
        let mut re: RegularExpression =
            RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("123abcd"), Ok(false));
    }

    #[test]
    fn ab_test_1() {
        let mut re: RegularExpression = RegularExpression::new("(a|b)*abb(a|b)*");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("aaaabbbbbb"), Ok(true));
    }

    #[test]
    fn easy_test_1() {
        let mut re: RegularExpression = RegularExpression::new("(a*|b*)*");
        re.generate_epsilon_nfa();
        assert_eq!(re.check(""), Ok(true));
    }

    #[test]
    fn counted_repetition_test_1() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){0}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check(""), Ok(true));
    }

    #[test]
    fn counted_repetition_test_2() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){0,0}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check(""), Ok(true));
    }

    #[test]
    fn counted_repetition_test_3() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){0,0}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("a"), Ok(false));
    }

    #[test]
    fn counted_repetition_test_4() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){0,1}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check(""), Ok(true));
    }

    #[test]
    fn counted_repetition_test_5() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){0,1}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("a"), Ok(true));
    }

    #[test]
    fn counted_repetition_test_6() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){0,1}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("ab"), Ok(false));
    }

    #[test]
    fn counted_repetition_test_7() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,4}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check(""), Ok(false));
    }

    #[test]
    fn counted_repetition_test_8() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,4}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("a"), Ok(false));
    }

    #[test]
    fn counted_repetition_test_9() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,4}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("ba"), Ok(true));
    }

    #[test]
    fn counted_repetition_test_10() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,4}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("aba"), Ok(true));
    }

    #[test]
    fn counted_repetition_test_11() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,4}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("aaba"), Ok(true));
    }

    #[test]
    fn counted_repetition_test_12() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,4}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("abbaa"), Ok(false));
    }

    #[test]
    fn counted_repetition_test_13() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2,}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("aaaaaaaaaaaa"), Ok(true));
    }

    #[test]
    fn counted_repetition_test_14() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("a"), Ok(false));
    }

    #[test]
    fn counted_repetition_test_15() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){2}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("abb"), Ok(false));
    }

    #[test]
    fn counted_repetition_test_16() {
        let mut re: RegularExpression = RegularExpression::new("(a|b){10,10}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("abaaa"), Ok(false));
    }

    #[test]
    fn email_test_1() {
        let mut re: RegularExpression =
            RegularExpression::new("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}");
        re.generate_epsilon_nfa();
        assert_eq!(re.check("john.smith@example.com"), Ok(true));
    }
}
