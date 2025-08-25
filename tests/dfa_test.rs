use dumb_regex::regex::RegularExpression;

#[test]
fn basic_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("a"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("a"));
}

#[test]
fn basic_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("a"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("b"));
}

#[test]
fn basic_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("a"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("ab"));
}

#[test]
fn star_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("a*"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("aaaaaaaaaaa"));
}

#[test]
fn star_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("a*"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("aaaaaaaaaabaaaaaa"));
}

#[test]
fn union_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("a|b|c"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("a"));
}

#[test]
fn union_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("a|b|c"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("b"));
}

#[test]
fn union_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("a|b|c"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("d"));
}

#[test]
fn char_set_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("[hc]at"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("hat"));
}

#[test]
fn char_set_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("[hc]at"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("cat"));
}

#[test]
fn char_set_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("[hc]at"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("mat"));
}

#[test]
fn any_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from(".at"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("hat"));
}

#[test]
fn any_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from(".at"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("cat"));
}

#[test]
fn any_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from(".at"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("mat"));
}

#[test]
fn any_test_4() {
    let re: RegularExpression = RegularExpression::new(String::from(".at"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("pat"));
}

#[test]
fn group_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("([hc]at)?[mp]at"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("mat"));
}

#[test]
fn group_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("([hc]at)?[mp]at"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("hat"));
}

#[test]
fn group_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("([hc]at)?[mp]at"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("pat"));
}

#[test]
fn group_test_4() {
    let re: RegularExpression = RegularExpression::new(String::from("([hc]at)?[mp]at"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("catmat"));
}

#[test]
fn set_range_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("[a-zA-Z0-9]"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("5"));
}

#[test]
fn set_range_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("[a-zA-Z0-9]"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("G"));
}

#[test]
fn set_range_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("[a-zA-Z0-9]"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("@"));
}

#[test]
fn negated_set_range_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("[^a-zA-Z0-9]"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("5"));
}

#[test]
fn negated_set_range_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("[^a-zA-Z0-9]"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("G"));
}

#[test]
fn negated_set_range_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("[^a-zA-Z0-9]"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("@"));
}

#[test]
fn special_char_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("\\w*"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("0123"));
}
#[test]
fn special_char_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("\\w*"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("ZYX"));
}

#[test]
fn special_char_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("\\w*"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("abcd"));
}

#[test]
fn special_char_test_4() {
    let re: RegularExpression = RegularExpression::new(String::from("\\w*"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("abcdef_ABCDEF___01234"));
}

#[test]
fn special_char_test_5() {
    let re: RegularExpression = RegularExpression::new(String::from("\\w*"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("0+1-2"));
}

#[test]
fn numeral_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("1"));
}

#[test]
fn numeral_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("1000000"));
}

#[test]
fn numeral_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("-1"));
}

#[test]
fn numeral_test_4() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("1e9"));
}

#[test]
fn numeral_test_5() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("1e-5"));
}

#[test]
fn numeral_test_6() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("1E-5"));
}

#[test]
fn numeral_test_7() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("1e-12233342"));
}

#[test]
fn numeral_test_8() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("3.1415926535"));
}

#[test]
fn numeral_test_9() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("237429342e24801"));
}

#[test]
fn numeral_test_10() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("6.022e+23"));
}

#[test]
fn numeral_test_11() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("e+23"));
}

#[test]
fn numeral_test_12() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("abcd"));
}

#[test]
fn numeral_test_13() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("abcd123"));
}

#[test]
fn numeral_test_14() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("123abcd"));
}

#[test]
fn ab_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b)*abb(a|b)*"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("aaaabbbbbb"));
}

#[test]
fn easy_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("(a*|b*)*"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match(""));
}

#[test]
fn counted_repetition_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){0}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match(""));
}

#[test]
fn counted_repetition_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){0,0}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match(""));
}

#[test]
fn counted_repetition_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){0,0}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("a"));
}

#[test]
fn counted_repetition_test_4() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){0,1}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match(""));
}

#[test]
fn counted_repetition_test_5() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){0,1}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("a"));
}

#[test]
fn counted_repetition_test_6() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){0,1}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("ab"));
}

#[test]
fn counted_repetition_test_7() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,4}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match(""));
}

#[test]
fn counted_repetition_test_8() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,4}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("a"));
}

#[test]
fn counted_repetition_test_9() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,4}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("ba"));
}

#[test]
fn counted_repetition_test_10() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,4}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("aba"));
}

#[test]
fn counted_repetition_test_11() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,4}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("aaba"));
}

#[test]
fn counted_repetition_test_12() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,4}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("abbaa"));
}

#[test]
fn counted_repetition_test_13() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("aaaaaaaaaaaa"));
}

#[test]
fn counted_repetition_test_14() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("a"));
}

#[test]
fn counted_repetition_test_15() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("abb"));
}

#[test]
fn counted_repetition_test_16() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){10,10}"));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(!dfa.is_match("abaaa"));
}

#[test]
fn email_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}",
    ));
    let dfa = re.to_epsilon_nfa().unwrap().to_nfa().to_dfa();

    assert!(dfa.is_match("john.smith@example.com"));
}
