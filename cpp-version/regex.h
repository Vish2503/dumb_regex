#ifndef REGEX_H
#define REGEX_H

#include <string>
#include <vector>
#include <map>
#include <set>
using namespace std;

/*
BNF Grammar of Regular Expressions

<RE>	::=	<simple-RE> <REtail>
<REtail> ::= "|" <simple-RE> <REtail> |  <empty>
<simple-RE>	::=	<basic-RE> <simple-REtail>
<simple-REtail> ::= <basic-RE> <simple-REtail> | <empty>
<basic-RE>	::=	<elementary-RE> "*" | <elementary-RE> "+" | <elementary-RE> "?" | <elementary-RE> "{n,m}" | <elementary-RE>
<elementary-RE>	::=	<group> | <any> | <char> | <set>
<group>	::=	"(" <RE> ")"
<any>	::=	"."
<char>	::=	any non metacharacter | "\" metacharacter
<set>	::=	"[" <set-items> "]" | "[^" <set-items> "]"
<set-items>	::=	<set-item> | <set-item> <set-items>
<set-item>	::=	<char> <range>
<range>	::=	"-" <char> | <empty>
<set-char>	::=	any character
*/ 
class RegularExpression {
private:
    enum EngineState {
        REGEX,
        EPSILON_NFA,
        NFA,
        DFA,
        MINIMIZED_DFA
    };
    EngineState engine_state = REGEX;

    const int epsilon = 256;
    vector<map<int, set<int>>> epsilon_nfa_transition; 
    pair<int, int> epsilon_nfa_start_end;
    int make_epsilon_nfa_node();

    vector<map<int, set<int>>> nfa_transition; 
    pair<int, set<int>> nfa_start_end;
    int make_nfa_node();

    vector<map<int, int>> dfa_transition; 
    pair<int, set<int>> dfa_start_end;
    int make_dfa_node();

    vector<map<int, int>> minimized_dfa_transition; 
    pair<int, set<int>> minimized_dfa_start_end;
    int make_minimized_dfa_node();

    string pattern;
    int parser_index = 0;

    int parser_peek();
    int parser_match(char c);
    int parser_match_one_of(string s);
    int parser_match_none_of(string s);

    pair<int, int> parse_RE();
    pair<int, int> parse_REtail(pair<int, int> lvalue);
    pair<int, int> parse_simple_RE();
    pair<int, int> parse_simple_REtail(pair<int, int> lvalue);
    pair<int, int> parse_basic_RE();
    pair<int, int> parse_elementary_RE();
    pair<int, int> parse_group();
    pair<int, int> parse_any();
    pair<int, int> parse_char();
    pair<int, int> parse_set();
    pair<int, int> parse_set_items();
    pair<int, int> parse_set_item();
    pair<int, int> parse_range(pair<int, int> lvalue);
    pair<int, int> parse_set_char();

    pair<int, int> make_deep_copy(int start, int end);

    void epsilon_closure(int curr, set<int>& res);

    void generate_epsilon_nfa();
    void generate_nfa();
    void generate_dfa();
    void generate_minimized_dfa();

    bool match_epsilon_nfa(string input);
    bool match_nfa(string input);
    bool match_dfa(string input);
    bool match_minimized_dfa(string input);
public:
    RegularExpression(string p);

    bool match(string input);

    void generate_graphviz_files();
};

#endif // REGEX_H