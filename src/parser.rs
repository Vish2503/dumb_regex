use std::{iter::Peekable, ops::RangeInclusive, str::Chars};

use crate::{
    StatePair,
    epsilon_nfa::{EpsilonNfa, EpsilonNfaBuilder},
};

pub struct Parser<'a> {
    pattern_iter: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(pattern: &'a str) -> Self {
        Parser {
            pattern_iter: pattern.chars().peekable(),
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

    fn parse_re(
        &mut self,
        epsilon_nfa_builder: &mut EpsilonNfaBuilder,
    ) -> Result<Option<StatePair>, String> {
        match self.parse_simple_re(epsilon_nfa_builder)? {
            Some(simple_re_res) => Ok(Some(
                self.parse_re_tail(epsilon_nfa_builder, simple_re_res)?,
            )),
            None => Ok(None),
        }
    }

    fn parse_re_tail(
        &mut self,
        epsilon_nfa_builder: &mut EpsilonNfaBuilder,
        lvalue: StatePair,
    ) -> Result<StatePair, String> {
        match self.parser_peek() {
            Some('|') => {
                self.parser_match('|')?;

                let Some(simple_re_res) = self.parse_simple_re(epsilon_nfa_builder)? else {
                    return Err("Unexpectedly found no simple_re after `|`".to_string());
                };

                let union_res = epsilon_nfa_builder.add_union_transition(lvalue, simple_re_res);
                self.parse_re_tail(epsilon_nfa_builder, union_res)
            }
            _ => Ok(lvalue),
        }
    }

    fn parse_simple_re(
        &mut self,
        epsilon_nfa_builder: &mut EpsilonNfaBuilder,
    ) -> Result<Option<StatePair>, String> {
        match self.parse_basic_re(epsilon_nfa_builder)? {
            Some(simple_re_res) => Ok(Some(
                self.parse_simple_re_tail(epsilon_nfa_builder, simple_re_res)?,
            )),
            None => Ok(None),
        }
    }

    fn parse_simple_re_tail(
        &mut self,
        epsilon_nfa_builder: &mut EpsilonNfaBuilder,
        lvalue: StatePair,
    ) -> Result<StatePair, String> {
        match self.parse_basic_re(epsilon_nfa_builder)? {
            Some(basic_re_res) => {
                let concat_res = epsilon_nfa_builder.add_concat_transition(lvalue, basic_re_res);
                self.parse_simple_re_tail(epsilon_nfa_builder, concat_res)
            }
            None => Ok(lvalue),
        }
    }

    fn parse_basic_re(
        &mut self,
        epsilon_nfa_builder: &mut EpsilonNfaBuilder,
    ) -> Result<Option<StatePair>, String> {
        let Some(elementary_re_res) = self.parse_elementary_re(epsilon_nfa_builder)? else {
            return Ok(None);
        };

        match self.parser_peek() {
            Some('*') => {
                self.parser_match('*')?;

                let star_res = epsilon_nfa_builder.add_star_transition(elementary_re_res);
                Ok(Some(star_res))
            }
            Some('+') => {
                self.parser_match('+')?;

                let plus_res = epsilon_nfa_builder.add_plus_transition(elementary_re_res);
                Ok(Some(plus_res))
            }
            Some('?') => {
                self.parser_match('?')?;

                let question_res = epsilon_nfa_builder.add_question_transition(elementary_re_res);
                Ok(Some(question_res))
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

                let start = epsilon_nfa_builder.add_state();
                let end = epsilon_nfa_builder.add_state();

                let (elementary_re_start, elementary_re_end) = elementary_re_res;

                if n == 0 {
                    epsilon_nfa_builder.add_epsilon_transition(start, end);
                }

                let mut repeated_elementary_re: Option<StatePair> = None;

                for _ in 1..=n {
                    let elementary_re_copy = epsilon_nfa_builder
                        .make_deep_copy(elementary_re_start, elementary_re_end)?;

                    if let Some(repeated_elementary_re_unwrapped) = repeated_elementary_re {
                        repeated_elementary_re = Some(epsilon_nfa_builder.add_concat_transition(
                            repeated_elementary_re_unwrapped,
                            elementary_re_copy,
                        ));
                    } else {
                        repeated_elementary_re = Some(elementary_re_copy);
                    }
                }

                if m == -1 {
                    let elementary_re_copy = epsilon_nfa_builder
                        .make_deep_copy(elementary_re_start, elementary_re_end)?;

                    let elementary_re_copy_star =
                        epsilon_nfa_builder.add_star_transition(elementary_re_copy);
                    if let Some(repeated_elementary_re_unwrapped) = repeated_elementary_re {
                        repeated_elementary_re = Some(epsilon_nfa_builder.add_concat_transition(
                            repeated_elementary_re_unwrapped,
                            elementary_re_copy_star,
                        ));
                    } else {
                        repeated_elementary_re = Some(elementary_re_copy_star);
                    }
                }

                for _ in n + 1..=m {
                    let elementary_re_copy = epsilon_nfa_builder
                        .make_deep_copy(elementary_re_start, elementary_re_end)?;

                    let elementary_re_copy_question =
                        epsilon_nfa_builder.add_question_transition(elementary_re_copy);
                    if let Some(repeated_elementary_re_unwrapped) = repeated_elementary_re {
                        repeated_elementary_re = Some(epsilon_nfa_builder.add_concat_transition(
                            repeated_elementary_re_unwrapped,
                            elementary_re_copy_question,
                        ));
                    } else {
                        repeated_elementary_re = Some(elementary_re_copy_question);
                    }
                }

                if let Some((repeated_elementary_re_start, repeated_elementary_re_end)) =
                    repeated_elementary_re
                {
                    epsilon_nfa_builder.add_epsilon_transition(start, repeated_elementary_re_start);
                    epsilon_nfa_builder.add_epsilon_transition(repeated_elementary_re_end, end);
                }

                Ok(Some((start, end)))
            }
            _ => Ok(Some(elementary_re_res)),
        }
    }

    fn parse_elementary_re(
        &mut self,
        epsilon_nfa_builder: &mut EpsilonNfaBuilder,
    ) -> Result<Option<StatePair>, String> {
        if let Some(group_res) = self.parse_group(epsilon_nfa_builder)? {
            return Ok(Some(group_res));
        }

        if let Some(any_res) = self.parse_any(epsilon_nfa_builder)? {
            return Ok(Some(any_res));
        }

        if let Some(char_res) = self.parse_char(epsilon_nfa_builder)? {
            return Ok(Some(char_res));
        }

        if let Some(set_res) = self.parse_set(epsilon_nfa_builder)? {
            return Ok(Some(set_res));
        }

        Ok(None)
    }

    fn parse_group(
        &mut self,
        epsilon_nfa_builder: &mut EpsilonNfaBuilder,
    ) -> Result<Option<StatePair>, String> {
        match self.parser_peek() {
            Some('(') => {
                self.parser_match('(')?;
                let re_res = self.parse_re(epsilon_nfa_builder)?;
                self.parser_match(')')?;
                Ok(re_res)
            }
            _ => Ok(None),
        }
    }

    fn parse_any(
        &mut self,
        epsilon_nfa_builder: &mut EpsilonNfaBuilder,
    ) -> Result<Option<StatePair>, String> {
        match self.parser_peek() {
            Some('.') => {
                self.parser_match('.')?;

                let start = epsilon_nfa_builder.add_state();
                let end = epsilon_nfa_builder.add_state();

                epsilon_nfa_builder.add_transition_range(
                    start,
                    u8::MIN as char..=u8::MAX as char,
                    end,
                );

                Ok(Some((start, end)))
            }
            _ => Ok(None),
        }
    }

    fn parse_char(
        &mut self,
        epsilon_nfa_builder: &mut EpsilonNfaBuilder,
    ) -> Result<Option<StatePair>, String> {
        let meta_characters = "[]\\.^$*+?{}|()";
        let possible_escape_characters = "[]\\.^$*+?{}|()wWsSdDnrt";
        let white_space = "\t\n\r ";
        match self.parser_peek() {
            Some('\\') => {
                self.parser_match('\\')?;
                match self.parser_match_one_of(possible_escape_characters)? {
                    c if meta_characters.contains(c) => {
                        let start = epsilon_nfa_builder.add_state();
                        let end = epsilon_nfa_builder.add_state();
                        epsilon_nfa_builder.add_transition(start, c, end);

                        Ok(Some((start, end)))
                    }
                    'w' => {
                        let start = epsilon_nfa_builder.add_state();
                        let end = epsilon_nfa_builder.add_state();

                        epsilon_nfa_builder.add_transition_range(
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
                        let start = epsilon_nfa_builder.add_state();
                        let end = epsilon_nfa_builder.add_state();

                        epsilon_nfa_builder.add_transition_range(
                            start,
                            (u8::MIN..=u8::MAX)
                                .map(|c| c as char)
                                .filter(|c| !c.is_alphanumeric() && *c != '_'),
                            end,
                        );

                        Ok(Some((start, end)))
                    }
                    's' => {
                        let start = epsilon_nfa_builder.add_state();
                        let end = epsilon_nfa_builder.add_state();

                        epsilon_nfa_builder.add_transition_range(start, white_space.chars(), end);

                        Ok(Some((start, end)))
                    }
                    'S' => {
                        let start = epsilon_nfa_builder.add_state();
                        let end = epsilon_nfa_builder.add_state();

                        epsilon_nfa_builder.add_transition_range(
                            start,
                            (u8::MIN..=u8::MAX)
                                .map(|c| c as char)
                                .filter(|&c| !white_space.contains(c)),
                            end,
                        );

                        Ok(Some((start, end)))
                    }
                    'd' => {
                        let start = epsilon_nfa_builder.add_state();
                        let end = epsilon_nfa_builder.add_state();

                        epsilon_nfa_builder.add_transition_range(start, '0'..='9', end);

                        Ok(Some((start, end)))
                    }
                    'D' => {
                        let start = epsilon_nfa_builder.add_state();
                        let end = epsilon_nfa_builder.add_state();

                        epsilon_nfa_builder.add_transition_range(
                            start,
                            (u8::MIN..=u8::MAX)
                                .map(|c| c as char)
                                .filter(|c| !c.is_ascii_digit()),
                            end,
                        );

                        Ok(Some((start, end)))
                    }
                    'n' => {
                        let start = epsilon_nfa_builder.add_state();
                        let end = epsilon_nfa_builder.add_state();
                        epsilon_nfa_builder.add_transition(start, '\n', end);

                        Ok(Some((start, end)))
                    }
                    'r' => {
                        let start = epsilon_nfa_builder.add_state();
                        let end = epsilon_nfa_builder.add_state();
                        epsilon_nfa_builder.add_transition(start, '\r', end);

                        Ok(Some((start, end)))
                    }
                    't' => {
                        let start = epsilon_nfa_builder.add_state();
                        let end = epsilon_nfa_builder.add_state();
                        epsilon_nfa_builder.add_transition(start, '\t', end);

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

                let start = epsilon_nfa_builder.add_state();
                let end = epsilon_nfa_builder.add_state();
                epsilon_nfa_builder.add_transition(start, c, end);

                Ok(Some((start, end)))
            }
            None => Ok(None),
        }
    }

    fn parse_set(
        &mut self,
        epsilon_nfa_builder: &mut EpsilonNfaBuilder,
    ) -> Result<Option<StatePair>, String> {
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

                let start = epsilon_nfa_builder.add_state();
                let end = epsilon_nfa_builder.add_state();

                if negate {
                    epsilon_nfa_builder.add_transition_range(
                        start,
                        (u8::MIN..=u8::MAX)
                            .map(|c| c as char)
                            .filter(|&c| !range.iter().any(|r| r.contains(&c))),
                        end,
                    );
                } else {
                    epsilon_nfa_builder.add_transition_range(
                        start,
                        range.into_iter().flatten(),
                        end,
                    );
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

    pub fn parse(&mut self) -> Result<EpsilonNfa, String> {
        let mut epsilon_nfa_builder = EpsilonNfaBuilder::new();

        match self.parse_re(&mut epsilon_nfa_builder)? {
            Some((epsilon_nfa_start, epsilon_nfa_end)) => Ok(EpsilonNfa::new(
                epsilon_nfa_builder.transitions,
                epsilon_nfa_start,
                epsilon_nfa_end,
            )),
            None => Err("Could not generate epsilon nfa".to_string()),
        }
    }
}
