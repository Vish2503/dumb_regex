use crate::regex::RegularExpression;

#[test]
fn basic_test_1() {
    let re: RegularExpression = RegularExpression::new("a");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("a"), Ok(true));
    assert_eq!(nfa.check("a"), Ok(true));
    assert_eq!(dfa.check("a"), Ok(true));
    assert_eq!(minimized_dfa.check("a"), Ok(true));
}

#[test]
fn basic_test_2() {
    let re: RegularExpression = RegularExpression::new("a");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("b"), Ok(false));
    assert_eq!(nfa.check("b"), Ok(false));
    assert_eq!(dfa.check("b"), Ok(false));
    assert_eq!(minimized_dfa.check("b"), Ok(false));
}

#[test]
fn basic_test_3() {
    let re: RegularExpression = RegularExpression::new("a");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("ab"), Ok(false));
    assert_eq!(nfa.check("ab"), Ok(false));
    assert_eq!(dfa.check("ab"), Ok(false));
    assert_eq!(minimized_dfa.check("ab"), Ok(false));
}

#[test]
fn star_test_1() {
    let re: RegularExpression = RegularExpression::new("a*");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("aaaaaaaaaaa"), Ok(true));
    assert_eq!(nfa.check("aaaaaaaaaaa"), Ok(true));
    assert_eq!(dfa.check("aaaaaaaaaaa"), Ok(true));
    assert_eq!(minimized_dfa.check("aaaaaaaaaaa"), Ok(true));
}

#[test]
fn star_test_2() {
    let re: RegularExpression = RegularExpression::new("a*");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("aaaaaaaaaabaaaaaa"), Ok(false));
    assert_eq!(nfa.check("aaaaaaaaaabaaaaaa"), Ok(false));
    assert_eq!(dfa.check("aaaaaaaaaabaaaaaa"), Ok(false));
    assert_eq!(minimized_dfa.check("aaaaaaaaaabaaaaaa"), Ok(false));
}

#[test]
fn union_test_1() {
    let re: RegularExpression = RegularExpression::new("a|b|c");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("a"), Ok(true));
    assert_eq!(nfa.check("a"), Ok(true));
    assert_eq!(dfa.check("a"), Ok(true));
    assert_eq!(minimized_dfa.check("a"), Ok(true));
}

#[test]
fn union_test_2() {
    let re: RegularExpression = RegularExpression::new("a|b|c");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("b"), Ok(true));
    assert_eq!(nfa.check("b"), Ok(true));
    assert_eq!(dfa.check("b"), Ok(true));
    assert_eq!(minimized_dfa.check("b"), Ok(true));
}

#[test]
fn union_test_3() {
    let re: RegularExpression = RegularExpression::new("a|b|c");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("d"), Ok(false));
    assert_eq!(nfa.check("d"), Ok(false));
    assert_eq!(dfa.check("d"), Ok(false));
    assert_eq!(minimized_dfa.check("d"), Ok(false));
}

#[test]
fn char_set_test_1() {
    let re: RegularExpression = RegularExpression::new("[hc]at");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("hat"), Ok(true));
    assert_eq!(nfa.check("hat"), Ok(true));
    assert_eq!(dfa.check("hat"), Ok(true));
    assert_eq!(minimized_dfa.check("hat"), Ok(true));
}

#[test]
fn char_set_test_2() {
    let re: RegularExpression = RegularExpression::new("[hc]at");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("cat"), Ok(true));
    assert_eq!(nfa.check("cat"), Ok(true));
    assert_eq!(dfa.check("cat"), Ok(true));
    assert_eq!(minimized_dfa.check("cat"), Ok(true));
}

#[test]
fn char_set_test_3() {
    let re: RegularExpression = RegularExpression::new("[hc]at");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("mat"), Ok(false));
    assert_eq!(nfa.check("mat"), Ok(false));
    assert_eq!(dfa.check("mat"), Ok(false));
    assert_eq!(minimized_dfa.check("mat"), Ok(false));
}

#[test]
fn any_test_1() {
    let re: RegularExpression = RegularExpression::new(".at");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("hat"), Ok(true));
    assert_eq!(nfa.check("hat"), Ok(true));
    assert_eq!(dfa.check("hat"), Ok(true));
    assert_eq!(minimized_dfa.check("hat"), Ok(true));
}

#[test]
fn any_test_2() {
    let re: RegularExpression = RegularExpression::new(".at");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("cat"), Ok(true));
    assert_eq!(nfa.check("cat"), Ok(true));
    assert_eq!(dfa.check("cat"), Ok(true));
    assert_eq!(minimized_dfa.check("cat"), Ok(true));
}

#[test]
fn any_test_3() {
    let re: RegularExpression = RegularExpression::new(".at");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("mat"), Ok(true));
    assert_eq!(nfa.check("mat"), Ok(true));
    assert_eq!(dfa.check("mat"), Ok(true));
    assert_eq!(minimized_dfa.check("mat"), Ok(true));
}

#[test]
fn any_test_4() {
    let re: RegularExpression = RegularExpression::new(".at");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("pat"), Ok(true));
    assert_eq!(nfa.check("pat"), Ok(true));
    assert_eq!(dfa.check("pat"), Ok(true));
    assert_eq!(minimized_dfa.check("pat"), Ok(true));
}

#[test]
fn group_test_1() {
    let re: RegularExpression = RegularExpression::new("([hc]at)?[mp]at");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("mat"), Ok(true));
    assert_eq!(nfa.check("mat"), Ok(true));
    assert_eq!(dfa.check("mat"), Ok(true));
    assert_eq!(minimized_dfa.check("mat"), Ok(true));
}

#[test]
fn group_test_2() {
    let re: RegularExpression = RegularExpression::new("([hc]at)?[mp]at");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("hat"), Ok(false));
    assert_eq!(nfa.check("hat"), Ok(false));
    assert_eq!(dfa.check("hat"), Ok(false));
    assert_eq!(minimized_dfa.check("hat"), Ok(false));
}

#[test]
fn group_test_3() {
    let re: RegularExpression = RegularExpression::new("([hc]at)?[mp]at");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("pat"), Ok(true));
    assert_eq!(nfa.check("pat"), Ok(true));
    assert_eq!(dfa.check("pat"), Ok(true));
    assert_eq!(minimized_dfa.check("pat"), Ok(true));
}

#[test]
fn group_test_4() {
    let re: RegularExpression = RegularExpression::new("([hc]at)?[mp]at");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("catmat"), Ok(true));
    assert_eq!(nfa.check("catmat"), Ok(true));
    assert_eq!(dfa.check("catmat"), Ok(true));
    assert_eq!(minimized_dfa.check("catmat"), Ok(true));
}

#[test]
fn set_range_test_1() {
    let re: RegularExpression = RegularExpression::new("[a-zA-Z0-9]");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("5"), Ok(true));
    assert_eq!(nfa.check("5"), Ok(true));
    assert_eq!(dfa.check("5"), Ok(true));
    assert_eq!(minimized_dfa.check("5"), Ok(true));
}

#[test]
fn set_range_test_2() {
    let re: RegularExpression = RegularExpression::new("[a-zA-Z0-9]");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("G"), Ok(true));
    assert_eq!(nfa.check("G"), Ok(true));
    assert_eq!(dfa.check("G"), Ok(true));
    assert_eq!(minimized_dfa.check("G"), Ok(true));
}

#[test]
fn set_range_test_3() {
    let re: RegularExpression = RegularExpression::new("[a-zA-Z0-9]");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("@"), Ok(false));
    assert_eq!(nfa.check("@"), Ok(false));
    assert_eq!(dfa.check("@"), Ok(false));
    assert_eq!(minimized_dfa.check("@"), Ok(false));
}

#[test]
fn negated_set_range_test_1() {
    let re: RegularExpression = RegularExpression::new("[^a-zA-Z0-9]");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("5"), Ok(false));
    assert_eq!(nfa.check("5"), Ok(false));
    assert_eq!(dfa.check("5"), Ok(false));
    assert_eq!(minimized_dfa.check("5"), Ok(false));
}

#[test]
fn negated_set_range_test_2() {
    let re: RegularExpression = RegularExpression::new("[^a-zA-Z0-9]");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("G"), Ok(false));
    assert_eq!(nfa.check("G"), Ok(false));
    assert_eq!(dfa.check("G"), Ok(false));
    assert_eq!(minimized_dfa.check("G"), Ok(false));
}

#[test]
fn negated_set_range_test_3() {
    let re: RegularExpression = RegularExpression::new("[^a-zA-Z0-9]");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("@"), Ok(true));
    assert_eq!(nfa.check("@"), Ok(true));
    assert_eq!(dfa.check("@"), Ok(true));
    assert_eq!(minimized_dfa.check("@"), Ok(true));
}

#[test]
fn special_char_test_1() {
    let re: RegularExpression = RegularExpression::new("\\w*");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("0123"), Ok(true));
    assert_eq!(nfa.check("0123"), Ok(true));
    assert_eq!(dfa.check("0123"), Ok(true));
    assert_eq!(minimized_dfa.check("0123"), Ok(true));
}
#[test]
fn special_char_test_2() {
    let re: RegularExpression = RegularExpression::new("\\w*");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("ZYX"), Ok(true));
    assert_eq!(nfa.check("ZYX"), Ok(true));
    assert_eq!(dfa.check("ZYX"), Ok(true));
    assert_eq!(minimized_dfa.check("ZYX"), Ok(true));
}

#[test]
fn special_char_test_3() {
    let re: RegularExpression = RegularExpression::new("\\w*");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("abcd"), Ok(true));
    assert_eq!(nfa.check("abcd"), Ok(true));
    assert_eq!(dfa.check("abcd"), Ok(true));
    assert_eq!(minimized_dfa.check("abcd"), Ok(true));
}

#[test]
fn special_char_test_4() {
    let re: RegularExpression = RegularExpression::new("\\w*");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("abcdef_ABCDEF___01234"), Ok(true));
    assert_eq!(nfa.check("abcdef_ABCDEF___01234"), Ok(true));
    assert_eq!(dfa.check("abcdef_ABCDEF___01234"), Ok(true));
    assert_eq!(minimized_dfa.check("abcdef_ABCDEF___01234"), Ok(true));
}

#[test]
fn special_char_test_5() {
    let re: RegularExpression = RegularExpression::new("\\w*");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("0+1-2"), Ok(false));
    assert_eq!(nfa.check("0+1-2"), Ok(false));
    assert_eq!(dfa.check("0+1-2"), Ok(false));
    assert_eq!(minimized_dfa.check("0+1-2"), Ok(false));
}

#[test]
fn numeral_test_1() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("1"), Ok(true));
    assert_eq!(nfa.check("1"), Ok(true));
    assert_eq!(dfa.check("1"), Ok(true));
    assert_eq!(minimized_dfa.check("1"), Ok(true));
}

#[test]
fn numeral_test_2() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("1000000"), Ok(true));
    assert_eq!(nfa.check("1000000"), Ok(true));
    assert_eq!(dfa.check("1000000"), Ok(true));
    assert_eq!(minimized_dfa.check("1000000"), Ok(true));
}

#[test]
fn numeral_test_3() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("-1"), Ok(true));
    assert_eq!(nfa.check("-1"), Ok(true));
    assert_eq!(dfa.check("-1"), Ok(true));
    assert_eq!(minimized_dfa.check("-1"), Ok(true));
}

#[test]
fn numeral_test_4() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("1e9"), Ok(true));
    assert_eq!(nfa.check("1e9"), Ok(true));
    assert_eq!(dfa.check("1e9"), Ok(true));
    assert_eq!(minimized_dfa.check("1e9"), Ok(true));
}

#[test]
fn numeral_test_5() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("1e-5"), Ok(true));
    assert_eq!(nfa.check("1e-5"), Ok(true));
    assert_eq!(dfa.check("1e-5"), Ok(true));
    assert_eq!(minimized_dfa.check("1e-5"), Ok(true));
}

#[test]
fn numeral_test_6() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("1E-5"), Ok(true));
    assert_eq!(nfa.check("1E-5"), Ok(true));
    assert_eq!(dfa.check("1E-5"), Ok(true));
    assert_eq!(minimized_dfa.check("1E-5"), Ok(true));
}

#[test]
fn numeral_test_7() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("1e-12233342"), Ok(true));
    assert_eq!(nfa.check("1e-12233342"), Ok(true));
    assert_eq!(dfa.check("1e-12233342"), Ok(true));
    assert_eq!(minimized_dfa.check("1e-12233342"), Ok(true));
}

#[test]
fn numeral_test_8() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("3.1415926535"), Ok(true));
    assert_eq!(nfa.check("3.1415926535"), Ok(true));
    assert_eq!(dfa.check("3.1415926535"), Ok(true));
    assert_eq!(minimized_dfa.check("3.1415926535"), Ok(true));
}

#[test]
fn numeral_test_9() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("237429342e24801"), Ok(true));
    assert_eq!(nfa.check("237429342e24801"), Ok(true));
    assert_eq!(dfa.check("237429342e24801"), Ok(true));
    assert_eq!(minimized_dfa.check("237429342e24801"), Ok(true));
}

#[test]
fn numeral_test_10() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("6.022e+23"), Ok(true));
    assert_eq!(nfa.check("6.022e+23"), Ok(true));
    assert_eq!(dfa.check("6.022e+23"), Ok(true));
    assert_eq!(minimized_dfa.check("6.022e+23"), Ok(true));
}

#[test]
fn numeral_test_11() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("e+23"), Ok(false));
    assert_eq!(nfa.check("e+23"), Ok(false));
    assert_eq!(dfa.check("e+23"), Ok(false));
    assert_eq!(minimized_dfa.check("e+23"), Ok(false));
}

#[test]
fn numeral_test_12() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("abcd"), Ok(false));
    assert_eq!(nfa.check("abcd"), Ok(false));
    assert_eq!(dfa.check("abcd"), Ok(false));
    assert_eq!(minimized_dfa.check("abcd"), Ok(false));
}

#[test]
fn numeral_test_13() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("abcd123"), Ok(false));
    assert_eq!(nfa.check("abcd123"), Ok(false));
    assert_eq!(dfa.check("abcd123"), Ok(false));
    assert_eq!(minimized_dfa.check("abcd123"), Ok(false));
}

#[test]
fn numeral_test_14() {
    let re: RegularExpression =
        RegularExpression::new("[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("123abcd"), Ok(false));
    assert_eq!(nfa.check("123abcd"), Ok(false));
    assert_eq!(dfa.check("123abcd"), Ok(false));
    assert_eq!(minimized_dfa.check("123abcd"), Ok(false));
}

#[test]
fn ab_test_1() {
    let re: RegularExpression = RegularExpression::new("(a|b)*abb(a|b)*");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("aaaabbbbbb"), Ok(true));
    assert_eq!(nfa.check("aaaabbbbbb"), Ok(true));
    assert_eq!(dfa.check("aaaabbbbbb"), Ok(true));
    assert_eq!(minimized_dfa.check("aaaabbbbbb"), Ok(true));
}

#[test]
fn easy_test_1() {
    let re: RegularExpression = RegularExpression::new("(a*|b*)*");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check(""), Ok(true));
    assert_eq!(nfa.check(""), Ok(true));
    assert_eq!(dfa.check(""), Ok(true));
    assert_eq!(minimized_dfa.check(""), Ok(true));
}

#[test]
fn counted_repetition_test_1() {
    let re: RegularExpression = RegularExpression::new("(a|b){0}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check(""), Ok(true));
    assert_eq!(nfa.check(""), Ok(true));
    assert_eq!(dfa.check(""), Ok(true));
    assert_eq!(minimized_dfa.check(""), Ok(true));
}

#[test]
fn counted_repetition_test_2() {
    let re: RegularExpression = RegularExpression::new("(a|b){0,0}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check(""), Ok(true));
    assert_eq!(nfa.check(""), Ok(true));
    assert_eq!(dfa.check(""), Ok(true));
    assert_eq!(minimized_dfa.check(""), Ok(true));
}

#[test]
fn counted_repetition_test_3() {
    let re: RegularExpression = RegularExpression::new("(a|b){0,0}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("a"), Ok(false));
    assert_eq!(nfa.check("a"), Ok(false));
    assert_eq!(dfa.check("a"), Ok(false));
    assert_eq!(minimized_dfa.check("a"), Ok(false));
}

#[test]
fn counted_repetition_test_4() {
    let re: RegularExpression = RegularExpression::new("(a|b){0,1}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check(""), Ok(true));
    assert_eq!(nfa.check(""), Ok(true));
    assert_eq!(dfa.check(""), Ok(true));
    assert_eq!(minimized_dfa.check(""), Ok(true));
}

#[test]
fn counted_repetition_test_5() {
    let re: RegularExpression = RegularExpression::new("(a|b){0,1}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("a"), Ok(true));
    assert_eq!(nfa.check("a"), Ok(true));
    assert_eq!(dfa.check("a"), Ok(true));
    assert_eq!(minimized_dfa.check("a"), Ok(true));
}

#[test]
fn counted_repetition_test_6() {
    let re: RegularExpression = RegularExpression::new("(a|b){0,1}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("ab"), Ok(false));
    assert_eq!(nfa.check("ab"), Ok(false));
    assert_eq!(dfa.check("ab"), Ok(false));
    assert_eq!(minimized_dfa.check("ab"), Ok(false));
}

#[test]
fn counted_repetition_test_7() {
    let re: RegularExpression = RegularExpression::new("(a|b){2,4}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check(""), Ok(false));
    assert_eq!(nfa.check(""), Ok(false));
    assert_eq!(dfa.check(""), Ok(false));
    assert_eq!(minimized_dfa.check(""), Ok(false));
}

#[test]
fn counted_repetition_test_8() {
    let re: RegularExpression = RegularExpression::new("(a|b){2,4}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("a"), Ok(false));
    assert_eq!(nfa.check("a"), Ok(false));
    assert_eq!(dfa.check("a"), Ok(false));
    assert_eq!(minimized_dfa.check("a"), Ok(false));
}

#[test]
fn counted_repetition_test_9() {
    let re: RegularExpression = RegularExpression::new("(a|b){2,4}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("ba"), Ok(true));
    assert_eq!(nfa.check("ba"), Ok(true));
    assert_eq!(dfa.check("ba"), Ok(true));
    assert_eq!(minimized_dfa.check("ba"), Ok(true));
}

#[test]
fn counted_repetition_test_10() {
    let re: RegularExpression = RegularExpression::new("(a|b){2,4}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("aba"), Ok(true));
    assert_eq!(nfa.check("aba"), Ok(true));
    assert_eq!(dfa.check("aba"), Ok(true));
    assert_eq!(minimized_dfa.check("aba"), Ok(true));
}

#[test]
fn counted_repetition_test_11() {
    let re: RegularExpression = RegularExpression::new("(a|b){2,4}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("aaba"), Ok(true));
    assert_eq!(nfa.check("aaba"), Ok(true));
    assert_eq!(dfa.check("aaba"), Ok(true));
    assert_eq!(minimized_dfa.check("aaba"), Ok(true));
}

#[test]
fn counted_repetition_test_12() {
    let re: RegularExpression = RegularExpression::new("(a|b){2,4}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("abbaa"), Ok(false));
    assert_eq!(nfa.check("abbaa"), Ok(false));
    assert_eq!(dfa.check("abbaa"), Ok(false));
    assert_eq!(minimized_dfa.check("abbaa"), Ok(false));
}

#[test]
fn counted_repetition_test_13() {
    let re: RegularExpression = RegularExpression::new("(a|b){2,}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("aaaaaaaaaaaa"), Ok(true));
    assert_eq!(nfa.check("aaaaaaaaaaaa"), Ok(true));
    assert_eq!(dfa.check("aaaaaaaaaaaa"), Ok(true));
    assert_eq!(minimized_dfa.check("aaaaaaaaaaaa"), Ok(true));
}

#[test]
fn counted_repetition_test_14() {
    let re: RegularExpression = RegularExpression::new("(a|b){2}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("a"), Ok(false));
    assert_eq!(nfa.check("a"), Ok(false));
    assert_eq!(dfa.check("a"), Ok(false));
    assert_eq!(minimized_dfa.check("a"), Ok(false));
}

#[test]
fn counted_repetition_test_15() {
    let re: RegularExpression = RegularExpression::new("(a|b){2}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("abb"), Ok(false));
    assert_eq!(nfa.check("abb"), Ok(false));
    assert_eq!(dfa.check("abb"), Ok(false));
    assert_eq!(minimized_dfa.check("abb"), Ok(false));
}

#[test]
fn counted_repetition_test_16() {
    let re: RegularExpression = RegularExpression::new("(a|b){10,10}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("abaaa"), Ok(false));
    assert_eq!(nfa.check("abaaa"), Ok(false));
    assert_eq!(dfa.check("abaaa"), Ok(false));
    assert_eq!(minimized_dfa.check("abaaa"), Ok(false));
}

#[test]
fn email_test_1() {
    let re: RegularExpression =
        RegularExpression::new("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}");
    let epsilon_nfa = re.to_epsilon_nfa();
    let nfa = epsilon_nfa.to_nfa();
    let dfa = nfa.to_dfa();
    let minimized_dfa = dfa.to_minimized_dfa();

    assert_eq!(epsilon_nfa.check("john.smith@example.com"), Ok(true));
    assert_eq!(nfa.check("john.smith@example.com"), Ok(true));
    assert_eq!(dfa.check("john.smith@example.com"), Ok(true));
    assert_eq!(minimized_dfa.check("john.smith@example.com"), Ok(true));
}
