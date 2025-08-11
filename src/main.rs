use std::{
    collections::{HashMap, HashSet},
    iter::Peekable,
    process::exit,
    str::Chars,
};

#[derive(Debug, Eq, PartialEq, Hash)]
enum Alphabet {
    Char(char),
    Epsilon,
}
type StateId = usize;
type StatePair = (StateId, StateId);
type NFATransition = HashMap<Alphabet, HashSet<StateId>>;

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

    fn invert_transition(&mut self, from: StateId, to: StateId) {
        for c in u8::MIN..=u8::MAX {
            match self.transitions[from].remove(&Alphabet::Char(c as char))
            {
                Some(_) => continue,
                None => {
                    self.transitions[from]
                        .insert(Alphabet::Char(c as char), HashSet::from([to]));
                }
            }
        }
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
        todo!()
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

        todo!()
    }

    fn parse_set_item(&mut self) -> Result<Option<StatePair>, String> {
        todo!()
    }

    fn parse_range(&mut self) -> Result<Option<StatePair>, String> {
        todo!()
    }

    fn parse_set_char(&mut self) -> Result<Option<StatePair>, String> {
        todo!()
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
}

fn main() {
    let pattern: &str = ".";
    let mut re: RegularExpression = RegularExpression::new(pattern);
    re.generate_epsilon_nfa();

    println!("Hello, world!");
}
