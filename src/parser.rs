use std::{iter::Peekable, ops::RangeInclusive, str::Chars};

use crate::{StateId, StatePair, epsilon_nfa::EpsilonNfa};

pub struct Parser<'a> {
    pattern_iter: Peekable<Chars<'a>>,
    epsilon_nfa: EpsilonNfa,
}

impl<'a> Parser<'a> {
    pub fn new(pattern: &'a str) -> Self {
        Parser {
            pattern_iter: pattern.chars().peekable(),
            epsilon_nfa: EpsilonNfa::new(),
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

                self.epsilon_nfa.add_transition_range(
                    start,
                    u8::MIN as char..=u8::MAX as char,
                    end,
                );

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

                let Some(range) = self.parse_set_items()? else {
                    return Err("Empty character set found in the pattern.".to_string());
                };

                self.parser_match(']')?;

                let start = self.epsilon_nfa.add_state();
                let end = self.epsilon_nfa.add_state();

                if negate {
                    self.epsilon_nfa.add_transition_range(
                        start,
                        (u8::MIN..=u8::MAX)
                            .map(|c| c as char)
                            .filter(|&c| !range.iter().any(|r| r.contains(&c))),
                        end,
                    );
                } else {
                    self.epsilon_nfa
                        .add_transition_range(start, range.into_iter().flatten(), end);
                }

                Ok(Some((start, end)))
            }
            _ => Ok(None),
        }
    }

    fn parse_set_items(&mut self) -> Result<Option<Vec<RangeInclusive<char>>>, String> {
        let Some(set_item_res) = self.parse_set_item()? else {
            return Ok(None);
        };

        let Some(mut set_items_res) = self.parse_set_items()? else {
            return Ok(Some(set_item_res));
        };

        set_items_res.extend(set_item_res);

        Ok(Some(set_items_res))
    }

    fn parse_set_item(&mut self) -> Result<Option<Vec<RangeInclusive<char>>>, String> {
        match self.parse_set_char()? {
            Some(char_res) => Ok(Some(self.parse_range(char_res)?)),
            _ => Ok(None),
        }
    }

    fn parse_range(&mut self, lvalue: char) -> Result<Vec<RangeInclusive<char>>, String> {
        let Some('-') = self.parser_peek() else {
            return Ok(vec![lvalue..=lvalue]);
        };

        self.parser_match('-')?;

        match self.parse_set_char()? {
            None => Ok(vec![lvalue..=lvalue, '-'..='-']),
            Some(char_res) => {
                let range_start = lvalue;
                let range_end = char_res;

                if range_start > range_end {
                    Ok(vec![
                        range_start..=range_start,
                        '-'..='-',
                        range_end..=range_end,
                    ])
                } else {
                    Ok(vec![range_start..=range_end])
                }
            }
        }
    }

    fn parse_set_char(&mut self) -> Result<Option<char>, String> {
        let meta_characters = "[]\\";
        let possible_escape_characters = "[]\\nrt";
        match self.parser_peek() {
            Some('\\') => {
                self.parser_match('\\')?;
                match self.parser_match_one_of(possible_escape_characters)? {
                    c if meta_characters.contains(c) => Ok(Some(c)),
                    'n' => Ok(Some('\n')),
                    'r' => Ok(Some('\r')),
                    't' => Ok(Some('\t')),
                    _ => Err("Unexpected behaviour in parse_char()".to_string()),
                }
            }
            Some(c) => {
                if meta_characters.contains(c) {
                    Ok(None)
                } else {
                    let c = self.parser_match_none_of(meta_characters)?;
                    Ok(Some(c))
                }
            }
            None => Ok(None),
        }
    }

    pub fn parse(&mut self) -> Result<Option<EpsilonNfa>, String> {
        match self.parse_re()? {
            Some((start, end)) => {
                self.epsilon_nfa.start = Some(start);
                self.epsilon_nfa.end = Some(end);
                Ok(Some(self.epsilon_nfa.clone()))
            }
            None => Ok(None),
        }
    }
}
