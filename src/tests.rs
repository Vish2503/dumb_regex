use crate::regex::RegularExpression;

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
fn set_range_test_3() {
    let mut re: RegularExpression = RegularExpression::new("[a-zA-Z0-9]");
    re.generate();
    assert_eq!(re.check("@"), Ok(false));
}

#[test]
fn negated_set_range_test_1() {
    let mut re: RegularExpression = RegularExpression::new("[^a-zA-Z0-9]");
    re.generate();
    assert_eq!(re.check("5"), Ok(false));
}

#[test]
fn negated_set_range_test_2() {
    let mut re: RegularExpression = RegularExpression::new("[^a-zA-Z0-9]");
    re.generate();
    assert_eq!(re.check("G"), Ok(false));
}

#[test]
fn negated_set_range_test_3() {
    let mut re: RegularExpression = RegularExpression::new("[^a-zA-Z0-9]");
    re.generate();
    assert_eq!(re.check("@"), Ok(true));
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
