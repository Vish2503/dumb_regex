#include <iostream>
#include <vector>
#include <unordered_map>
#include <set>
#include <cassert>
using namespace std;

/*
BNF Grammar of Regular Expressions

<RE>	::=	<simple-RE> <REtail>
<REtail> ::= "|" <simple-RE> <REtail> |  <empty>
<simple-RE>	::=	<basic-RE> <simple-REtail>
<simple-REtail> ::= <basic-RE> <simple-REtail> | <empty>
<basic-RE>	::=	<elementary-RE> "*" | <elementary-RE> "+" | <elementary-RE> "?" | <elementary-RE>
<elementary-RE>	::=	<group> | <any> | <char> | <set>
<group>	::=	"(" <RE> ")"
<any>	::=	"."
<set>	::=	"[" <set-items> "]" | "[^" <set-items> "]"
<set-items>	::=	<set-item> | <set-item> <set-items>
<set-item>	::=	<char> <range>
<range>	::=	"-" <char> | <empty>
<char>	::=	any non metacharacter | "\" metacharacter
*/ 
class RegularExpression {
private:
    string pattern;
    int parser_index = 0;

    const int epsilon = 256;
    vector<unordered_map<int, set<int>>> epsilon_nfa_transition; 
    pair<int, int> epsilon_nfa_start_end;

    int make_epsilon_nfa_node() {
        int node = epsilon_nfa_transition.size();
        epsilon_nfa_transition.push_back(unordered_map<int, set<int>>());
        return node;
    }

    vector<unordered_map<int, set<int>>> nfa_transition; 
    pair<int, int> nfa_start_end;

    int parser_peek() {
        if (parser_index == (int) pattern.size()) {
            return -1;
        }
        
        return int(pattern[parser_index]);
    }

    int parser_match(char c) {
        if (parser_peek() != c) {
            cerr << "Expected `" << c << "` but found `" << parser_peek() << "` at index " << parser_index << endl;
            exit(1);
        }
    
        return int(pattern[parser_index++]);
    }

    int parser_match_one_of(string s) {
        if (s.find(parser_peek()) == string::npos) {
            cerr << "Expected one of `" << s << "` but found `" << char(parser_peek()) << "` at index " << parser_index << endl;
            exit(1);
        }
    
        return int(pattern[parser_index++]);
    }
    
    int parser_match_none_of(string s) {
        if (0 <= parser_peek() && parser_peek() < 256 && s.find(parser_peek()) != string::npos) {
            cerr << "Expected not one of `" << s << "` but found `" << char(parser_peek()) << "` at index " << parser_index << endl;
            exit(1);
        }
    
        return int(pattern[parser_index++]);
    }

    pair<int, int> parse_RE() {
        pair<int, int> simple_RE_res = parse_simple_RE();
        if (simple_RE_res == pair<int, int>{-1, -1}) // not meant to be parsed here
            return {-1, -1};

        return parse_REtail(simple_RE_res);
    }

    pair<int, int> parse_REtail(pair<int, int> lvalue) {
        if (parser_peek() == '|') { 
            parser_match('|');

            pair<int, int> simple_RE_res = parse_simple_RE();
            
            int start = make_epsilon_nfa_node();
            int end = make_epsilon_nfa_node();

            auto [up_start, up_end] = lvalue;
            auto [down_start, down_end] = simple_RE_res;

            epsilon_nfa_transition[start][epsilon].insert(up_start);
            epsilon_nfa_transition[up_end][epsilon].insert(end);
            
            epsilon_nfa_transition[start][epsilon].insert(down_start);
            epsilon_nfa_transition[down_end][epsilon].insert(end);

            pair<int, int> new_res = make_pair(start, end);

            return parse_REtail(new_res);
        } else { // <empty> case
            return lvalue; 
        }

        assert(false);
        return {-1, -1};
    }

    pair<int, int> parse_simple_RE() {
        pair<int, int> basic_RE_res = parse_basic_RE();
        if (basic_RE_res == pair<int, int>{-1, -1}) // not meant to be parsed here
            return {-1, -1};

        return parse_simple_REtail(basic_RE_res);
    }

    pair<int, int> parse_simple_REtail(pair<int, int> lvalue) {
        pair<int, int> basic_RE_res = parse_basic_RE();
        if (basic_RE_res == pair<int, int>{-1, -1}) // <empty> case
            return lvalue; 

        auto [left_start, left_end] = lvalue;
        auto [right_start, right_end] = basic_RE_res;

        epsilon_nfa_transition[left_end][epsilon].insert(right_start); // concatenation

        pair<int, int> right_res = make_pair(left_start, right_end);

        return parse_simple_REtail(right_res);
    }

    pair<int, int> parse_basic_RE() {
        pair<int, int> elementary_RE_res = parse_elementary_RE();
        if (elementary_RE_res == pair<int, int>{-1, -1}) // not meant to be parsed here
            return {-1, -1}; 

        if (parser_peek() != '*' && parser_peek() != '+' && parser_peek() != '?')
            return elementary_RE_res;

        auto [elementary_RE_start, elementary_RE_end] = elementary_RE_res;

        int start = make_epsilon_nfa_node();
        int end = make_epsilon_nfa_node();

        // common for all
        epsilon_nfa_transition[start][epsilon].insert(elementary_RE_start);
        epsilon_nfa_transition[elementary_RE_end][epsilon].insert(end);

        if (parser_peek() == '*') {
            parser_match('*');
            
            epsilon_nfa_transition[elementary_RE_end][epsilon].insert(elementary_RE_start);
            epsilon_nfa_transition[start][epsilon].insert(end);
        } else if (parser_peek() == '+') {
            parser_match('+');
            
            epsilon_nfa_transition[elementary_RE_end][epsilon].insert(elementary_RE_start);
        } else if (parser_peek() == '?') {
            parser_match('?');
            
            epsilon_nfa_transition[start][epsilon].insert(end);
        } else {
            assert(false);
        }

        return {start, end};
    }

    pair<int, int> parse_elementary_RE() {
        pair<int, int> group_res = parse_group();
        if (group_res != pair<int, int>{-1, -1})
            return group_res;

        pair<int, int> any_res = parse_any();
        if (any_res != pair<int, int>{-1, -1})
            return any_res;
        
        pair<int, int> char_res = parse_char();
        if (char_res != pair<int, int>{-1, -1})
            return char_res;

        pair<int, int> set_res = parse_set();
        if (set_res != pair<int, int>{-1, -1})
            return set_res;

        // not meant to be parsed here 
        return {-1, -1};
    }

    pair<int, int> parse_group() {
        if (parser_peek() == '(') {
            parser_match('(');
            pair<int, int> RE_res = parse_RE();
            parser_match(')');
            return RE_res;
        }
        
        // not meant to be parsed here 
        return {-1, -1};
    }

    pair<int, int> parse_any() {
        if (parser_peek() == '.') {
            parser_match('.');

            int start = make_epsilon_nfa_node();
            int end = make_epsilon_nfa_node();
            
            // transition for all ascii characters
            for (int c = 0; c < 256; c++)
                epsilon_nfa_transition[start][c].insert(end);

            return {start, end};
        }
        
        // not meant to be parsed here
        return {-1, -1};
    }

    pair<int, int> parse_set() {
        if (parser_peek() == '[') {
            parser_match('[');

            bool negate = false;
            if (parser_peek() == '^') {
                // negative set
                parser_match('^');
                negate = true;
            }

            pair<int, int> set_items_res = parse_set_items();
            if (set_items_res == pair<int, int>{-1, -1}) {
                cerr << "Empty character set found in the pattern at index " << parser_index << endl;
                exit(1);
            }
            parser_match(']');

            auto [start, end] = set_items_res;
            if (negate) {
                unordered_map<int, set<int>> old = epsilon_nfa_transition[start];
                epsilon_nfa_transition[start].clear();
                for (int c = 0; c < 256; c++)
                    if (old.find(c) == old.end())
                        epsilon_nfa_transition[start][c].insert(end);
            }

            return {start, end};
        }
        
        // not meant to be parsed here
        return {-1, -1};
    }

    pair<int, int> parse_set_items() {
        pair<int, int> set_item_res = parse_set_item();
        if (set_item_res == pair<int, int>{-1, -1})
            return {-1, -1};

        pair<int, int> set_items_res = parse_set_items();
        if (set_items_res == pair<int, int>{-1, -1})
            return set_item_res;

        auto [start, end] = set_item_res;
        for (auto &[c, _]: epsilon_nfa_transition[set_items_res.first])
            epsilon_nfa_transition[start][c].insert(end);  

        return {start, end};
    }

    pair<int, int> parse_set_item() {
        pair<int, int> char_res = parse_char();
        if (char_res == pair<int, int>{-1, -1})
            return {-1, -1}; // not meant to be parsed here

        pair<int, int> range_res = parse_range(char_res);
        return range_res;
    }

    pair<int, int> parse_range(pair<int, int> lvalue) {
        if (parser_peek() == '-') {
            parser_match('-');

            auto [start, end] = lvalue;

            pair<int, int> char_res = parse_char();

            if (char_res == pair<int, int>{-1, -1}) { // treat '-' as a normal character
                epsilon_nfa_transition[start]['-'].insert(end);
                
                return {start, end};
            }
            
            char range_start = epsilon_nfa_transition[start].begin()->first;
            char range_end = epsilon_nfa_transition[char_res.first].begin()->first;
            
            if (range_start > range_end) { // treat as normal characters, for example: z-a as 'z' '-' 'a'
                epsilon_nfa_transition[start][range_start].insert(end);
                epsilon_nfa_transition[start]['-'].insert(end);
                epsilon_nfa_transition[start][range_end].insert(end);
            } else {
                for (int c = range_start; c <= range_end; c++)
                    epsilon_nfa_transition[start][c].insert(end);
            }

            return {start, end};
        }
        
        return lvalue; // <empty> case
    }

    pair<int, int> parse_char() {
        const string meta_characters = "[]\\.^$*+?{}|()";
        if (parser_peek() == '\\') {
            parser_match('\\');

            int c = parser_match_one_of(meta_characters);

            int start = make_epsilon_nfa_node();
            int end = make_epsilon_nfa_node();
            epsilon_nfa_transition[start][c].insert(end);

            return {start, end};
        } else if (0 <= parser_peek() && parser_peek() < 256 && meta_characters.find(parser_peek()) == string::npos) {
            int c = parser_match_none_of(meta_characters);

            int start = make_epsilon_nfa_node();
            int end = make_epsilon_nfa_node();
            epsilon_nfa_transition[start][c].insert(end);

            return {start, end};
        }

        // parser_peek() is not meant to be parsed here
        return {-1, -1};
    }

    void epsilon_closure(int curr, set<int>& res) {
        res.insert(curr);
        for (auto next: epsilon_nfa_transition[curr][epsilon]) {
            if (res.find(next) == res.end())
                epsilon_closure(next, res);
        }
    }

public:
    RegularExpression(string pattern) : pattern(pattern) {
        epsilon_nfa_start_end = parse_RE();
    }

    bool match(string input) {
        auto [start, end] = epsilon_nfa_start_end;

        set<int> epsilon_closure_start;
        epsilon_closure(start, epsilon_closure_start);

        set<int> current_states;
        current_states.insert(epsilon_closure_start.begin(), epsilon_closure_start.end());
        for (auto c: input) {
            set<int> next_states;
            for (auto curr: current_states) {
                for (auto next: epsilon_nfa_transition[curr][c]) {
                    set<int> next_epsilon_closure;
                    epsilon_closure(next, next_epsilon_closure);
                    next_states.insert(next_epsilon_closure.begin(), next_epsilon_closure.end());
                }
            }
            current_states = next_states;
        }

        return (current_states.find(end) != current_states.end());
    }
};


void run_testcases() {
    vector<pair<string, string>> testcases = {
        {"a", "a"},
        {"a", "b"},
        {"a", "ab"},
        {"a*", "aaaaaaaaaaa"},
        {"a*", "aaaaaaaaaabaaaaaa"},
        {"a|b|c", "a"},
        {"a|b|c", "b"},
        {"a|b|c", "d"},
        {"[hc]at", "hat"},
        {"[hc]at", "cat"},
        {"[hc]at", "mat"},
        {".at", "hat"},
        {".at", "cat"},
        {".at", "mat"},
        {".at", "pat"},
        {"([hc]at)?[mp]at", "hat"},
        {"([hc]at)?[mp]at", "mat"},
        {"([hc]at)?[mp]at", "catmat"},
        {"([hc]at)?[mp]at", "pat"},
        {"[a-zA-Z0-9]", "G"},
        {"[a-zA-Z0-9]", "5"},
        {"[a-zA-Z0-9]", "@"},
        // Regular Expression for matching a numeral (https://en.wikipedia.org/wiki/Regular_expression)
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "1"},
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "1000000"},
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "-1"},
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "1e9"},
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "1e-5"},
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "1E-5"},
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "1e-12233342"},
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "3.1415926535"},
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "237429342e24801"},
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "6.022e+23"},
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "e+23"},
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "abcd"},
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "abcd123"},
        {"[\\+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][\\+-]?[0-9]+)?", "123abcd"},
    };

    for (auto [pattern, input]: testcases) {
        RegularExpression regex(pattern);
        bool is_match = regex.match(input);
        cout << pattern << " " << input << ": " << (is_match? "match": "no match") << endl;
    }
}

int main() {

    run_testcases();

    return 0;
}