use dumb_regex::regex::RegularExpression;

#[test]
fn basic_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("a"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("a"));
}

#[test]
fn basic_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("a"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("b"));
}

#[test]
fn basic_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("a"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("ab"));
}

#[test]
fn star_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("a*"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("aaaaaaaaaaa"));
}

#[test]
fn star_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("a*"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("aaaaaaaaaabaaaaaa"));
}

#[test]
fn union_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("a|b|c"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("a"));
}

#[test]
fn union_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("a|b|c"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("b"));
}

#[test]
fn union_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("a|b|c"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("d"));
}

#[test]
fn char_set_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("[hc]at"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("hat"));
}

#[test]
fn char_set_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("[hc]at"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("cat"));
}

#[test]
fn char_set_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("[hc]at"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("mat"));
}

#[test]
fn any_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from(".at"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("hat"));
}

#[test]
fn any_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from(".at"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("cat"));
}

#[test]
fn any_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from(".at"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("mat"));
}

#[test]
fn any_test_4() {
    let re: RegularExpression = RegularExpression::new(String::from(".at"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("pat"));
}

#[test]
fn group_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("([hc]at)?[mp]at"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("mat"));
}

#[test]
fn group_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("([hc]at)?[mp]at"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("hat"));
}

#[test]
fn group_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("([hc]at)?[mp]at"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("pat"));
}

#[test]
fn group_test_4() {
    let re: RegularExpression = RegularExpression::new(String::from("([hc]at)?[mp]at"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("catmat"));
}

#[test]
fn set_range_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("[a-zA-Z0-9]"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("5"));
}

#[test]
fn set_range_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("[a-zA-Z0-9]"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("G"));
}

#[test]
fn set_range_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("[a-zA-Z0-9]"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("@"));
}

#[test]
fn negated_set_range_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("[^a-zA-Z0-9]"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("5"));
}

#[test]
fn negated_set_range_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("[^a-zA-Z0-9]"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("G"));
}

#[test]
fn negated_set_range_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("[^a-zA-Z0-9]"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("@"));
}

#[test]
fn special_char_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("\\w*"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("0123"));
}
#[test]
fn special_char_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("\\w*"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("ZYX"));
}

#[test]
fn special_char_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("\\w*"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("abcd"));
}

#[test]
fn special_char_test_4() {
    let re: RegularExpression = RegularExpression::new(String::from("\\w*"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("abcdef_ABCDEF___01234"));
}

#[test]
fn special_char_test_5() {
    let re: RegularExpression = RegularExpression::new(String::from("\\w*"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("0+1-2"));
}

#[test]
fn numeral_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("1"));
}

#[test]
fn numeral_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("1000000"));
}

#[test]
fn numeral_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("-1"));
}

#[test]
fn numeral_test_4() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("1e9"));
}

#[test]
fn numeral_test_5() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("1e-5"));
}

#[test]
fn numeral_test_6() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("1E-5"));
}

#[test]
fn numeral_test_7() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("1e-12233342"));
}

#[test]
fn numeral_test_8() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("3.1415926535"));
}

#[test]
fn numeral_test_9() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("237429342e24801"));
}

#[test]
fn numeral_test_10() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("6.022e+23"));
}

#[test]
fn numeral_test_11() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("e+23"));
}

#[test]
fn numeral_test_12() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("abcd"));
}

#[test]
fn numeral_test_13() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("abcd123"));
}

#[test]
fn numeral_test_14() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("123abcd"));
}

#[test]
fn ab_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b)*abb(a|b)*"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("aaaabbbbbb"));
}

#[test]
fn easy_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("(a*|b*)*"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match(""));
}

#[test]
fn counted_repetition_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){0}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match(""));
}

#[test]
fn counted_repetition_test_2() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){0,0}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match(""));
}

#[test]
fn counted_repetition_test_3() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){0,0}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("a"));
}

#[test]
fn counted_repetition_test_4() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){0,1}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match(""));
}

#[test]
fn counted_repetition_test_5() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){0,1}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("a"));
}

#[test]
fn counted_repetition_test_6() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){0,1}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("ab"));
}

#[test]
fn counted_repetition_test_7() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,4}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match(""));
}

#[test]
fn counted_repetition_test_8() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,4}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("a"));
}

#[test]
fn counted_repetition_test_9() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,4}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("ba"));
}

#[test]
fn counted_repetition_test_10() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,4}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("aba"));
}

#[test]
fn counted_repetition_test_11() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,4}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("aaba"));
}

#[test]
fn counted_repetition_test_12() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,4}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("abbaa"));
}

#[test]
fn counted_repetition_test_13() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2,}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("aaaaaaaaaaaa"));
}

#[test]
fn counted_repetition_test_14() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("a"));
}

#[test]
fn counted_repetition_test_15() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){2}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("abb"));
}

#[test]
fn counted_repetition_test_16() {
    let re: RegularExpression = RegularExpression::new(String::from("(a|b){10,10}"));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(!nfa.is_match("abaaa"));
}

#[test]
fn email_test_1() {
    let re: RegularExpression = RegularExpression::new(String::from(
        "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}",
    ));
    let nfa = re.to_epsilon_nfa().unwrap().to_nfa();

    assert!(nfa.is_match("john.smith@example.com"));
}
