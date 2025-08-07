#include <iostream>
#include <vector>
#include <map>
#include <algorithm>
#include <set>
#include <cassert>
#include <fstream>
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
    int make_epsilon_nfa_node() {
        int node = epsilon_nfa_transition.size();
        epsilon_nfa_transition.push_back(map<int, set<int>>());
        return node;
    }

    vector<map<int, set<int>>> nfa_transition; 
    pair<int, set<int>> nfa_start_end;
    int make_nfa_node() {
        int node = nfa_transition.size();
        nfa_transition.push_back(map<int, set<int>>());
        return node;
    }

    vector<map<int, int>> dfa_transition; 
    pair<int, set<int>> dfa_start_end;
    int make_dfa_node() {
        int node = dfa_transition.size();
        dfa_transition.push_back(map<int, int>());
        return node;
    }

    vector<map<int, int>> minimized_dfa_transition; 
    pair<int, set<int>> minimized_dfa_start_end;
    int make_minimized_dfa_node() {
        int node = minimized_dfa_transition.size();
        minimized_dfa_transition.push_back(map<int, int>());
        return node;
    }

    string pattern;
    int parser_index = 0;

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

        if (parser_peek() != '*' && parser_peek() != '+' && parser_peek() != '?' && parser_peek() != '{')
            return elementary_RE_res;

        auto [elementary_RE_start, elementary_RE_end] = elementary_RE_res;

        int start = make_epsilon_nfa_node();
        int end = make_epsilon_nfa_node();

        if (parser_peek() == '*') {
            parser_match('*');

            epsilon_nfa_transition[start][epsilon].insert(elementary_RE_start);
            epsilon_nfa_transition[elementary_RE_end][epsilon].insert(end);
            
            epsilon_nfa_transition[elementary_RE_end][epsilon].insert(elementary_RE_start);
            epsilon_nfa_transition[start][epsilon].insert(end);
        } else if (parser_peek() == '+') {
            parser_match('+');

            epsilon_nfa_transition[start][epsilon].insert(elementary_RE_start);
            epsilon_nfa_transition[elementary_RE_end][epsilon].insert(end);
            
            epsilon_nfa_transition[elementary_RE_end][epsilon].insert(elementary_RE_start);
        } else if (parser_peek() == '?') {
            parser_match('?');
            
            epsilon_nfa_transition[start][epsilon].insert(elementary_RE_start);
            epsilon_nfa_transition[elementary_RE_end][epsilon].insert(end);

            epsilon_nfa_transition[start][epsilon].insert(end);
        } else if (parser_peek() == '{') {
            parser_match('{');

            const string digits = "0123456789";

            int n = 0, m = 0;
            while ('0' <= parser_peek() && parser_peek() <= '9') {
                int c = parser_match_one_of(digits);
                n = n * 10 + (c - '0');
            }

            if (parser_peek() == ',') {
                parser_match(',');

                if ('0' <= parser_peek() && parser_peek() <= '9') {
                    m = 0;
                    while ('0' <= parser_peek() && parser_peek() <= '9') {
                        int c = parser_match_one_of(digits);
                        m = m * 10 + (c - '0');
                    }
                } else {
                    m = -1; // placeholder for {n,}
                }
            } else {
                m = n;
            }

            parser_match('}');

            if (m != -1 && m < n) {
                cerr << "Out of order range found in the pattern at index " << parser_index << endl;
                exit(1);
            }

            if (n == 0) {
                epsilon_nfa_transition[start][epsilon].insert(end);
            }

            auto repeated_elementary_RE_start = -1;
            auto repeated_elementary_RE_end = -1;

            for (int i = 1; i <= n; i++) {
                auto [elementary_RE_copy_start, elementary_RE_copy_end] = make_deep_copy(elementary_RE_start, elementary_RE_end);

                if (repeated_elementary_RE_start == -1 && repeated_elementary_RE_end == -1) {
                    repeated_elementary_RE_start = elementary_RE_copy_start; 
                    repeated_elementary_RE_end = elementary_RE_copy_end; 
                } else {
                    epsilon_nfa_transition[repeated_elementary_RE_end][epsilon].insert(elementary_RE_copy_start);
                    repeated_elementary_RE_end = elementary_RE_copy_end;
                }
            }

            if (m == -1) {
                auto [elementary_RE_copy_start, elementary_RE_copy_end] = make_deep_copy(elementary_RE_start, elementary_RE_end);

                // similar to *
                int new_elementary_RE_copy_start = make_epsilon_nfa_node();
                int new_elementary_RE_copy_end = make_epsilon_nfa_node();

                epsilon_nfa_transition[new_elementary_RE_copy_start][epsilon].insert(elementary_RE_copy_start);
                epsilon_nfa_transition[elementary_RE_copy_end][epsilon].insert(new_elementary_RE_copy_end);

                epsilon_nfa_transition[elementary_RE_copy_end][epsilon].insert(elementary_RE_copy_start);
                epsilon_nfa_transition[new_elementary_RE_copy_start][epsilon].insert(new_elementary_RE_copy_end);

                if (repeated_elementary_RE_start == -1 && repeated_elementary_RE_end == -1) {
                    repeated_elementary_RE_start = new_elementary_RE_copy_start; 
                    repeated_elementary_RE_end = new_elementary_RE_copy_end; 
                } else {
                    epsilon_nfa_transition[repeated_elementary_RE_end][epsilon].insert(new_elementary_RE_copy_start);
                    repeated_elementary_RE_end = new_elementary_RE_copy_end;
                }
            }


            for (int i = n + 1; i <= m; i++) {
                auto [elementary_RE_copy_start, elementary_RE_copy_end] = make_deep_copy(elementary_RE_start, elementary_RE_end);

                // similar to ?
                int new_elementary_RE_copy_start = make_epsilon_nfa_node();
                int new_elementary_RE_copy_end = make_epsilon_nfa_node();

                epsilon_nfa_transition[new_elementary_RE_copy_start][epsilon].insert(elementary_RE_copy_start);
                epsilon_nfa_transition[elementary_RE_copy_end][epsilon].insert(new_elementary_RE_copy_end);

                epsilon_nfa_transition[new_elementary_RE_copy_start][epsilon].insert(new_elementary_RE_copy_end);

                if (repeated_elementary_RE_start == -1 && repeated_elementary_RE_end == -1) {
                    repeated_elementary_RE_start = new_elementary_RE_copy_start; 
                    repeated_elementary_RE_end = new_elementary_RE_copy_end; 
                } else {
                    epsilon_nfa_transition[repeated_elementary_RE_end][epsilon].insert(new_elementary_RE_copy_start);
                    repeated_elementary_RE_end = new_elementary_RE_copy_end;
                }
            }
            
            if (repeated_elementary_RE_start != -1 && repeated_elementary_RE_end != -1) {
                epsilon_nfa_transition[start][epsilon].insert(repeated_elementary_RE_start);
                epsilon_nfa_transition[repeated_elementary_RE_end][epsilon].insert(end);
            }
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

    pair<int, int> parse_char() {
        const string meta_characters = "[]\\.^$*+?{}|()"; // special characters which already have a special meaning before escaping
        const string special_characters = "wWsSdDabfnrtv"; // normal characters which have a special meaning after escaping
        const string white_space = "\t\n\f\r ";
        if (parser_peek() == '\\') {
            parser_match('\\');

            int c = parser_match_one_of(meta_characters + special_characters);

            if (meta_characters.find(c) != string::npos) {
                int start = make_epsilon_nfa_node();
                int end = make_epsilon_nfa_node();
                epsilon_nfa_transition[start][c].insert(end);
    
                return {start, end};
            } else if (special_characters.find(c) != string::npos) {
                int start = make_epsilon_nfa_node();
                int end = make_epsilon_nfa_node();
                switch (c) {
                    case 'w':
                        for (int i = 0; i < 256; i++) {
                            if (('a' <= i && i <= 'z') || ('A' <= i && i <= 'Z') || ('0' <= i && i <= '9') || (i == '_')) {
                                epsilon_nfa_transition[start][i].insert(end);
                            }
                        }
                        break;
                    case 'W':
                        for (int i = 0; i < 256; i++) {
                            if (!(('a' <= i && i <= 'z') || ('A' <= i && i <= 'Z') || ('0' <= i && i <= '9') || (i == '_'))) {
                                epsilon_nfa_transition[start][i].insert(end);
                            }
                        }
                        break;
                    case 's':
                        for (auto i: white_space) {
                            epsilon_nfa_transition[start][i].insert(end);
                        }
                        break;
                    case 'S':
                        for (int i = 0; i < 256; i++) {
                            if (white_space.find(char(i)) == string::npos) {
                                epsilon_nfa_transition[start][i].insert(end);
                            }
                        }
                        break;
                    case 'd':
                        for (int i = '0'; i <= '9'; i++) {
                            epsilon_nfa_transition[start][i].insert(end);
                        }
                        break;
                    case 'D':
                        for (int i = 0; i < 256; i++) {
                            if (!('0' <= i && i <= '9')) {
                                epsilon_nfa_transition[start][i].insert(end);
                            }
                        }
                        break;
                    case 'a':
                        epsilon_nfa_transition[start]['\a'].insert(end);
                        break;
                    case 'b':
                        epsilon_nfa_transition[start]['\b'].insert(end);
                        break;
                    case 'f':
                        epsilon_nfa_transition[start]['\f'].insert(end);
                        break;
                    case 'n':
                        epsilon_nfa_transition[start]['\n'].insert(end);
                        break;
                    case 'r':
                        epsilon_nfa_transition[start]['\r'].insert(end);
                        break;
                    case 't':
                        epsilon_nfa_transition[start]['\t'].insert(end);
                        break;
                    case 'v':
                        epsilon_nfa_transition[start]['\v'].insert(end);
                        break;
                    default:
                        assert(false);
                        break;
                }

                return {start, end};
            } else {
                assert(false);
            }
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
                map<int, set<int>> old = epsilon_nfa_transition[start];
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
        pair<int, int> char_res = parse_set_char();
        if (char_res == pair<int, int>{-1, -1})
            return {-1, -1}; // not meant to be parsed here

        pair<int, int> range_res = parse_range(char_res);
        return range_res;
    }

    pair<int, int> parse_range(pair<int, int> lvalue) {
        if (parser_peek() == '-') {
            parser_match('-');

            auto [start, end] = lvalue;

            pair<int, int> char_res = parse_set_char();

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

    pair<int, int> parse_set_char() {
        const string meta_characters = "[]\\"; // special characters which already have a special meaning before escaping
        const string special_characters = "abfnrtv"; // normal characters which have a special meaning after escaping
        if (parser_peek() == '\\') {
            parser_match('\\');

            int c = parser_match_one_of(meta_characters + special_characters);

            if (meta_characters.find(c) != string::npos) {
                int start = make_epsilon_nfa_node();
                int end = make_epsilon_nfa_node();
                epsilon_nfa_transition[start][c].insert(end);
    
                return {start, end};
            } else if (special_characters.find(c) != string::npos) {
                int start = make_epsilon_nfa_node();
                int end = make_epsilon_nfa_node();
                switch (c) {
                    case 'a':
                        epsilon_nfa_transition[start]['\a'].insert(end);
                        break;
                    case 'b':
                        epsilon_nfa_transition[start]['\b'].insert(end);
                        break;
                    case 'f':
                        epsilon_nfa_transition[start]['\f'].insert(end);
                        break;
                    case 'n':
                        epsilon_nfa_transition[start]['\n'].insert(end);
                        break;
                    case 'r':
                        epsilon_nfa_transition[start]['\r'].insert(end);
                        break;
                    case 't':
                        epsilon_nfa_transition[start]['\t'].insert(end);
                        break;
                    case 'v':
                        epsilon_nfa_transition[start]['\v'].insert(end);
                        break;
                    default:
                        assert(false);
                        break;
                }

                return {start, end};
            } else {
                assert(false);
            }
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

    pair<int, int> make_deep_copy(int start, int end) {
        map<int, int> mappings;
        mappings[start] = make_epsilon_nfa_node();

        vector<int> stack = {start};
        while (!stack.empty()) {
            int curr = stack.back();
            stack.pop_back();
            
            assert(mappings.find(curr) != mappings.end());

            for (int alphabet = 0; alphabet <= 256; alphabet++) {
                for (auto next: epsilon_nfa_transition[curr][alphabet]) {
                    if (mappings.find(next) == mappings.end()) {
                        mappings[next] = make_epsilon_nfa_node();
                        stack.push_back(next);
                    }
                    epsilon_nfa_transition[mappings[curr]][alphabet].insert(mappings[next]);
                }
            }
        }

        return make_pair(mappings[start], mappings[end]);
    }

    void epsilon_closure(int curr, set<int>& res) {
        res.insert(curr);
        for (auto next: epsilon_nfa_transition[curr][epsilon]) {
            if (res.find(next) == res.end())
                epsilon_closure(next, res);
        }
    }

    void generate_epsilon_nfa() {
        if (engine_state >= EPSILON_NFA)
            return;
        
        epsilon_nfa_start_end = parse_RE();
        engine_state = EPSILON_NFA;    
    }

    void generate_nfa() {
        if (engine_state >= NFA)
            return;
        
        generate_epsilon_nfa();

        int n = epsilon_nfa_transition.size();
        nfa_transition.resize(n); // one to one mapping between states

        auto [epsilon_nfa_start, epsilon_nfa_end] = epsilon_nfa_start_end;

        int nfa_start = epsilon_nfa_start;
        set<int> nfa_end;

        vector<set<int>> node_epsilon_closure(n);
        for (int curr = 0; curr < n; curr++) {
            set<int> curr_epsilon_closure;
            epsilon_closure(curr, curr_epsilon_closure);

            node_epsilon_closure[curr] = curr_epsilon_closure;

            if (curr_epsilon_closure.find(epsilon_nfa_end) != curr_epsilon_closure.end())
                nfa_end.insert(curr);
        }

        for (int curr = 0; curr < n; curr++) {
            for (auto epsilon_state: node_epsilon_closure[curr]) { // first epsilon closure
                for (auto &[alphabet, next_states]: epsilon_nfa_transition[epsilon_state]) { // input character
                    if (alphabet == epsilon)
                        continue;
                    for (auto next: next_states) {
                        nfa_transition[curr][alphabet].insert(node_epsilon_closure[next].begin(), node_epsilon_closure[next].end()); // second epsilon closure
                    }
                }
            }
        }

        nfa_start_end = make_pair(nfa_start, nfa_end);
        engine_state = NFA;
    }

    void generate_dfa() {
        if (engine_state >= DFA)
            return;
        
        generate_nfa();

        auto [nfa_start, nfa_end] = nfa_start_end;

        make_dfa_node(); // default dead state at 0

        map<set<int>, int> subset_to_dfa_node;

        set<int> start = {nfa_start};
        subset_to_dfa_node[start] = make_dfa_node();

        vector<set<int>> stack;
        stack.push_back(start);
        while (!stack.empty()) {
            set<int> curr_states = stack.back();
            stack.pop_back();
            int curr_node = subset_to_dfa_node[curr_states];

            map<int, set<int>> current_transitions;
            for (auto curr: curr_states) {
                for (auto &[alphabet, next_states]: nfa_transition[curr]) {
                    current_transitions[alphabet].insert(next_states.begin(), next_states.end());
                }
            }
            
            for (auto &[alphabet, next_states]: current_transitions) {
                if (subset_to_dfa_node.find(next_states) == subset_to_dfa_node.end()) {
                    subset_to_dfa_node[next_states] = make_dfa_node();
                    stack.push_back(next_states);
                }
                dfa_transition[curr_node][alphabet] = subset_to_dfa_node[next_states];
            }
        }

        set<int> end_states;
        for (auto &[subset, dfa_node]: subset_to_dfa_node) {
            for (auto end: nfa_end) {
                if (subset.find(end) != subset.end()) {
                    end_states.insert(dfa_node);
                    break;
                }
            }
        }

        dfa_start_end = make_pair(subset_to_dfa_node[start], end_states);
        engine_state = DFA;
    }

    void generate_minimized_dfa() {
        if (engine_state >= MINIMIZED_DFA)
            return;
        
        generate_dfa();

        auto [dfa_start, dfa_end] = dfa_start_end;
        int total_dfa_states = dfa_transition.size();

        // unreachable states
        set<int> reachable_states; {
            reachable_states.insert(dfa_start);
            set<int> current_states = {dfa_start};
            while (!current_states.empty()) {
                set<int> next_states;
                for (auto state: current_states) {
                    for (auto &[_, next]: dfa_transition[state])
                        if (reachable_states.find(next) == reachable_states.end())
                            next_states.insert(next);
                }
                reachable_states.insert(next_states.begin(), next_states.end());
                current_states = next_states;
            }
        }

        // dead states
        set<int> dead_states; { 
            for (int i = 1; i < total_dfa_states; i++) {
                set<int> current_states = {i};
                
                bool end_state_reachable = false;
                vector<int> stack = {i};
                while (!stack.empty()) {
                    int curr = stack.back();
                    stack.pop_back();
                    if (dfa_end.find(curr) != dfa_end.end()) {
                        end_state_reachable = true;
                        break;
                    }
                    for (auto [_, next]: dfa_transition[curr]) {
                        if (current_states.find(next) == current_states.end()) {
                            current_states.insert(next);
                            stack.push_back(next);
                        }
                    }
                }

                if (!end_state_reachable)
                    dead_states.insert(i);
            }
        }

        // non distinguishable states
        map<int, int> group_mapping;
        // initial set of groups is: {{single_dead_state}, {all_end_states}, {all_non_end_states}} we will refine this further 
        group_mapping[0] = 0; // dead state
        for (int i = 1; i < total_dfa_states; i++) { 
            if (reachable_states.find(i) == reachable_states.end())
                continue;
            if (dead_states.find(i) != dead_states.end())
                continue;

            if (dfa_end.find(i) != dfa_end.end())
                group_mapping[i] = 1;
            else
                group_mapping[i] = 2;
        }

        while (true) {
            bool change = false;
            for (int alphabet = 0; alphabet < 256; alphabet++) {
                map<pair<int, int>, set<int>> group_to_states;
                for (auto &[curr, _]: group_mapping) {
                    int next = dfa_transition[curr][alphabet];
                    group_to_states[make_pair(group_mapping[curr], group_mapping[next])].insert(curr);
                }

                map<int, int> new_group_mapping;
                int group_number = 0;
                for (auto &[_, states]: group_to_states) {
                    for (auto state: states)
                        new_group_mapping[state] = group_number;
                    group_number++;
                }

                if (new_group_mapping != group_mapping) {
                    group_mapping = new_group_mapping;
                    change = true;
                    break;
                }
            }

            if (!change)
                break;
        }

        make_minimized_dfa_node(); // default dead state at 0
        
        int largest_node = 0;
        for (auto &[_, group]: group_mapping)
            largest_node = max(largest_node, group);
        
        while (make_minimized_dfa_node() != largest_node);

        int minimized_dfa_start = -1; 
        set<int> minimized_dfa_end;
        for (auto [dfa_state, group]: group_mapping) { // each group is a state in minimized_dfa
            for (int alphabet = 0; alphabet < 256; alphabet++)
                minimized_dfa_transition[group][alphabet] = group_mapping[dfa_transition[dfa_state][alphabet]];

            if (dfa_state == dfa_start)
                minimized_dfa_start = group;

            if (dfa_end.find(dfa_state) != dfa_end.end())
                minimized_dfa_end.insert(group);
        }

        minimized_dfa_start_end = make_pair(minimized_dfa_start, minimized_dfa_end);
        engine_state = MINIMIZED_DFA;
    }

    bool match_epsilon_nfa(string input) {
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

    bool match_nfa(string input) {
        auto [start, end] = nfa_start_end;
    
        set<int> current_states = {start};
        for (auto c: input) {
            set<int> next_states;
            for (auto curr: current_states) {
                for (auto next: nfa_transition[curr][c]) {
                    next_states.insert(next);
                }
            }
            current_states = next_states;
        }

        for (auto end_state: end)
            if (current_states.find(end_state) != current_states.end())
                return true;

        return false;
    }

    bool match_dfa(string input) {
        auto [start, end] = dfa_start_end;

        int curr = start;
        for (auto c: input) {
            curr = dfa_transition[curr][c];
        }

        return end.find(curr) != end.end();
    }

    bool match_minimized_dfa(string input) {
        auto [start, end] = minimized_dfa_start_end;

        int curr = start;
        for (auto c: input) {
            curr = minimized_dfa_transition[curr][c];
        }

        return end.find(curr) != end.end();
    }
public:
    RegularExpression(string p) : pattern(p) {
        generate_minimized_dfa();
    }

    bool match(string input) {
        switch (engine_state) {
            case EPSILON_NFA:
                return match_epsilon_nfa(input);
            case NFA:
                return match_nfa(input);
            case DFA:
                return match_dfa(input);
            case MINIMIZED_DFA:
                return match_minimized_dfa(input);
            default:
                assert(false);
                return false;
        }
    }

    void generate_graphviz_files() {
        auto compress_label = [](string label) {
            sort(label.begin(), label.end());
            int n = label.size();
            
            string new_label;
            int start = 0;
            while (start < n) {
                int end = start + 1;
                while (end < n && label[end] == label[end - 1] + 1) end++;
                if (end - start == 1)
                    new_label += label[start];
                else if (end - start == 2)
                    new_label += string{label[start], ' ',label[end - 1]};
                else
                    new_label += string{label[start], '-', label[end - 1]};
                new_label += " ";
                start = end;
            }
            new_label.pop_back();

            return new_label;
        };
        
        if (engine_state >= EPSILON_NFA) {
            ofstream fout("graphviz/epsilon_nfa.gv");
            map<pair<int, int>, string> labels;
            for (int i = 0; i < (int) epsilon_nfa_transition.size(); i++) {
                for (auto [alphabet, next_states]: epsilon_nfa_transition[i]) {
                    for (auto next: next_states) {
                        if (next == 0) continue; // ignore transition to dead state
                        auto edge = make_pair(i, next);
                        labels[edge] += (alphabet == epsilon? "e": string(1, alphabet));
                    }
                }
            }
            
            // compress labels
            for (auto &[_, label]: labels) {
                label = compress_label(label);
            }
    
            fout << "digraph {\n\trankdir=LR;\n\tnode [shape = point]; _;\n\tnode [shape = doublecircle]; ";
            fout << epsilon_nfa_start_end.second;
            fout << ";\n\tnode [shape = circle];\n\t_ -> ";
            fout << epsilon_nfa_start_end.first << ";\n";
            for (auto [pair, l]: labels) {
                auto [curr, next] = pair;
                fout << '\t' << curr << " -> " << next << " [label = \"" << l << "\"];" << endl;
            }
            fout << "}" << endl;
    
            fout.close();
        } // epsilon_nfa
        
        if (engine_state >= NFA) {
            ofstream fout("graphviz/nfa.gv");
            map<pair<int, int>, string> labels;
            for (int i = 0; i < (int) nfa_transition.size(); i++) {
                for (auto [alphabet, next_states]: nfa_transition[i]) {
                    for (auto next: next_states) {
                        if (next == 0) continue; // ignore transition to dead state
                        auto edge = make_pair(i, next);
                        labels[edge] += (alphabet == epsilon? "e": string(1, alphabet));
                    }
                }
            }

            // compress labels
            for (auto &[_, label]: labels) {
                label = compress_label(label);
            }
    
            fout << "digraph {\n\trankdir=LR;\n\tnode [shape = point]; _;\n\tnode [shape = doublecircle]; ";
            for (auto end_state: nfa_start_end.second)
                fout << end_state << " ";
            fout << ";\n\tnode [shape = circle];\n\t_ -> ";
            fout << nfa_start_end.first << ";\n";
            for (auto [pair, l]: labels) {
                auto [curr, next] = pair;
                fout << '\t' << curr << " -> " << next << " [label = \"" << l << "\"];" << endl;
            }
            fout << "}" << endl;
    
            fout.close();
        } // nfa
        
        if (engine_state >= DFA) {
            ofstream fout("graphviz/dfa.gv");
            map<pair<int, int>, string> labels;
            for (int i = 0; i < (int) dfa_transition.size(); i++) {
                for (auto [alphabet, next]: dfa_transition[i]) {
                    if (next == 0) continue; // ignore transition to dead state
                    auto edge = make_pair(i, next);
                    labels[edge] += (alphabet == epsilon? "e": string(1, alphabet));
                }
            }

            // compress labels
            for (auto &[_, label]: labels) {
                label = compress_label(label);
            }
    
            fout << "digraph {\n\trankdir=LR;\n\tnode [shape = point]; _;\n\tnode [shape = doublecircle]; ";
            for (auto end_state: dfa_start_end.second)
                fout << end_state << " ";
            fout << ";\n\tnode [shape = circle];\n\t_ -> ";
            fout << dfa_start_end.first << ";\n";
            for (auto [pair, l]: labels) {
                auto [curr, next] = pair;
                fout << '\t' << curr << " -> " << next << " [label = \"" << l << "\"];" << endl;
            }
            fout << "}" << endl;
    
            fout.close();
        } // dfa
        
        if (engine_state >= MINIMIZED_DFA) {
            ofstream fout("graphviz/minimized_dfa.gv");
            map<pair<int, int>, string> labels;
            for (int i = 0; i < (int) minimized_dfa_transition.size(); i++) {
                for (auto [alphabet, next]: minimized_dfa_transition[i]) {
                    if (next == 0) continue; // ignore transition to dead state
                    auto edge = make_pair(i, next);
                    labels[edge] += (alphabet == epsilon? "e": string(1, alphabet));
                }
            }

            // compress labels
            for (auto &[_, label]: labels) {
                label = compress_label(label);
            }
    
            fout << "digraph {\n\trankdir=LR;\n\tnode [shape = point]; _;\n\tnode [shape = doublecircle]; ";
            for (auto end_state: minimized_dfa_start_end.second)
                fout << end_state << " ";
            fout << ";\n\tnode [shape = circle];\n\t_ -> ";
            fout << minimized_dfa_start_end.first << ";\n";
            for (auto [pair, l]: labels) {
                auto [curr, next] = pair;
                fout << '\t' << curr << " -> " << next << " [label = \"" << l << "\"];" << endl;
            }
            fout << "}" << endl;
    
            fout.close();
        } // minimized_dfa
    }
};


void run_testcases() {
    vector<pair<pair<string, string>, bool>> testcases = {
        {{"a", "a"}, true},
        {{"a", "b"}, false},
        {{"a", "ab"}, false},
        {{"a*", "aaaaaaaaaaa"}, true},
        {{"a*", "aaaaaaaaaabaaaaaa"}, false},
        {{"a|b|c", "a"}, true},
        {{"a|b|c", "b"}, true},
        {{"a|b|c", "d"}, false},
        {{"[hc]at", "hat"}, true},
        {{"[hc]at", "cat"}, true},
        {{"[hc]at", "mat"}, false},
        {{".at", "hat"}, true},
        {{".at", "cat"}, true},
        {{".at", "mat"}, true},
        {{".at", "pat"}, true},
        {{"([hc]at)?[mp]at", "mat"}, true},
        {{"([hc]at)?[mp]at", "hat"}, false,},
        {{"([hc]at)?[mp]at", "pat"}, true},
        {{"([hc]at)?[mp]at", "catmat"}, true},
        {{"[a-zA-Z0-9]", "5"}, true},
        {{"[a-zA-Z0-9]", "G"}, true},
        {{"\\w*", "0123"}, true},
        {{"\\w*", "ZYX"}, true},
        {{"\\w*", "abcd"}, true},
        {{"\\w*", "abcdef_ABCDEF___01234"}, true},
        {{"\\w*", "0+1-2"}, false},
        // Regular Expression for matching a numeral (https://en.wikipedia.org/wiki/Regular_expression)
        {{"[a-zA-Z0-9]", "@"}, false},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "1"}, true},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "1000000"}, true},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "-1"}, true},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "1e9"}, true},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "1e-5"}, true},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "1E-5"}, true},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "1e-12233342"}, true},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "3.1415926535"}, true},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "237429342e24801"}, true},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "6.022e+23"}, true},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "e+23"}, false},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "abcd"}, false},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "abcd123"}, false},
        {{"[+-]?(\\d+(\\.\\d*)?|\\.\\d+)([eE][+-]?\\d+)?", "123abcd"}, false},
        {{"(a|b)*abb(a|b)*", "aaaabbbbbb"}, true},
        {{"(a*|b*)*", ""}, true},
        {{"(a|b){0}", ""}, true},
        {{"(a|b){0,0}", ""}, true},
        {{"(a|b){0,0}", "a"}, false},
        {{"(a|b){0,1}", ""}, true},
        {{"(a|b){0,1}", "a"}, true},
        {{"(a|b){0,1}", "ab"}, false},
        {{"(a|b){2,4}", ""}, false},
        {{"(a|b){2,4}", "a"}, false},
        {{"(a|b){2,4}", "ba"}, true},
        {{"(a|b){2,4}", "aba"}, true},
        {{"(a|b){2,4}", "aaba"}, true},
        {{"(a|b){2,4}", "abbaa"}, false},
        {{"(a|b){2,}", "aaaaaaaaaaaa"}, true},
        {{"(a|b){2}", "a"}, false},
        {{"(a|b){2}", "abb"}, false},
        {{"(a|b){10,10}", "abaaa"}, false},
        {{"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}", "john.smith@example.com"}, true},
    };

    for (auto &[testcase, expected]: testcases) {
        auto [pattern, input] = testcase;
        RegularExpression regex(pattern);
        bool answer = regex.match(input);
        if (answer != expected) {
            cout << "Failed Testcase (" << pattern << ", " << input << ")\n"
                 << "\tExpected: " << (expected? "match": "no_match") 
                 << " but found " << (answer? "match": "no_match") << endl;
        } else {
            cout << "Passed Testcase (" << pattern << ", " << input << ")" << endl;
        }
    }
}

int main() {

    run_testcases();

    return 0;
}