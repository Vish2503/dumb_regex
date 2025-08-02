#include <bits/stdc++.h>
using namespace std;

const int eps = 256;

vector<unordered_map<int, set<int>>> adj; 

int make_node() {
    int node = adj.size();
    adj.push_back(unordered_map<int, set<int>>());
    return node;
}

string pattern;
int i = 0;

int peek() {
    if (i == (int) pattern.size()) {
        return -1;
    }
    
    return int(pattern[i]);
}

int match(char c) {
    if (peek() != c) {
        cerr << "Expected `" << c << "` but found `" << peek() << "` at index " << i << endl;
        exit(1);
    }

    return int(pattern[i++]);
}

int match_one_of(string s) {
    if (s.find(peek()) == string::npos) {
        cerr << "Expected one of `" << s << "` but found `" << char(peek()) << "` at index " << i << endl;
        exit(1);
    }

    return int(pattern[i++]);
}

int match_none_of(string s) {
    if (0 <= peek() && peek() < 256 && s.find(peek()) != string::npos) {
        cerr << "Expected not one of `" << s << "` but found `" << char(peek()) << "` at index " << i << endl;
        exit(1);
    }

    return int(pattern[i++]);
}

/*
https://www.cs.sfu.ca/~cameron/Teaching/384/99-3/regexp-plg.html
BNF Grammar of Regular Expressions
Following the precedence rules given previously, a BNF grammar for Perl-style regular expressions can be constructed as follows.
<RE>	::=	<union> | <simple-RE>
<union>	::=	<RE> "|" <simple-RE>
<simple-RE>	::=	<concatenation> | <basic-RE>
<concatenation>	::=	<simple-RE> <basic-RE>
<basic-RE>	::=	<star> | <plus> | <elementary-RE>
<star>	::=	<elementary-RE> "*"
<plus>	::=	<elementary-RE> "+"
<elementary-RE>	::=	<group> | <any> | <eos> | <char> | <set>
<group>	::=	"(" <RE> ")"
<any>	::=	"."
<eos>	::=	"$"
<char>	::=	any non metacharacter | "\" metacharacter
<set>	::=	<positive-set> | <negative-set>
<positive-set>	::=	"[" <set-items> "]"
<negative-set>	::=	"[^" <set-items> "]"
<set-items>	::=	<set-item> | <set-item> <set-items>
<set-item>	::=	<range> | <char>
<range>	::=	<char> "-" <char>

After making some changes:
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

pair<int, int> parse_RE();
pair<int, int> parse_REtail(pair<int, int> lvalue);
pair<int, int> parse_simple_RE();
pair<int, int> parse_simple_REtail(pair<int, int> lvalue);
pair<int, int> parse_basic_RE();
pair<int, int> parse_elementary_RE();
pair<int, int> parse_group();
pair<int, int> parse_any();
pair<int, int> parse_set();
pair<int, int> parse_set_items();
pair<int, int> parse_set_item();
pair<int, int> parse_range(pair<int, int> lvalue);
pair<int, int> parse_char();

pair<int, int> parse_RE() {
    pair<int, int> simple_RE_res = parse_simple_RE();
    if (simple_RE_res == pair<int, int>{-1, -1}) // not meant to be parsed here
        return {-1, -1};

    return parse_REtail(simple_RE_res);
}

pair<int, int> parse_REtail(pair<int, int> lvalue) {
    if (peek() == '|') { 
        match('|');

        pair<int, int> simple_RE_res = parse_simple_RE();
        
        int start = make_node();
        int end = make_node();

        auto [up_start, up_end] = lvalue;
        auto [down_start, down_end] = simple_RE_res;

        adj[start][eps].insert(up_start);
        adj[up_end][eps].insert(end);
        
        adj[start][eps].insert(down_start);
        adj[down_end][eps].insert(end);

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

    adj[left_end][eps].insert(right_start); // concatenation

    pair<int, int> right_res = make_pair(left_start, right_end);

    return parse_simple_REtail(right_res);
}

pair<int, int> parse_basic_RE() {
    pair<int, int> elementary_RE_res = parse_elementary_RE();
    if (elementary_RE_res == pair<int, int>{-1, -1}) // not meant to be parsed here
        return {-1, -1}; 

    if (peek() != '*' && peek() != '+' && peek() != '?')
        return elementary_RE_res;

    auto [elementary_RE_start, elementary_RE_end] = elementary_RE_res;

    int start = make_node();
    int end = make_node();

    // common for all
    adj[start][eps].insert(elementary_RE_start);
    adj[elementary_RE_end][eps].insert(end);

    if (peek() == '*') {
        match('*');
        
        adj[elementary_RE_end][eps].insert(elementary_RE_start);
        adj[start][eps].insert(end);
    } else if (peek() == '+') {
        match('+');
        
        adj[elementary_RE_end][eps].insert(elementary_RE_start);
    } else if (peek() == '?') {
        match('?');
        
        adj[start][eps].insert(end);
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
    if (peek() == '(') {
        match('(');
        pair<int, int> RE_res = parse_RE();
        match(')');
        return RE_res;
    }
    
    // not meant to be parsed here 
    return {-1, -1};
}

pair<int, int> parse_any() {
    if (peek() == '.') {
        match('.');

        int start = make_node();
        int end = make_node();
        
        // transition for all ascii characters
        for (int c = 0; c < 256; c++)
            adj[start][c].insert(end);

        return {start, end};
    }
    
    // not meant to be parsed here
    return {-1, -1};
}

pair<int, int> parse_set() {
    if (peek() == '[') {
        match('[');

        bool negate = false;
        if (peek() == '^') {
            // negative set
            match('^');
            negate = true;
        }

        pair<int, int> set_items_res = parse_set_items();
        if (set_items_res == pair<int, int>{-1, -1}) {
            cerr << "Empty character set found in the pattern at index " << i << endl;
            exit(1);
        }
        match(']');

        auto [start, end] = set_items_res;
        if (negate) {
            unordered_map<int, set<int>> old = adj[start];
            adj[start].clear();
            for (int c = 0; c < 256; c++)
                if (old.find(c) == old.end())
                    adj[start][c].insert(end);
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
    for (auto &[c, _]: adj[set_items_res.first])
        adj[start][c].insert(end);  

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
    if (peek() == '-') {
        match('-');

        auto [start, end] = lvalue;

        pair<int, int> char_res = parse_char();

        if (char_res == pair<int, int>{-1, -1}) { // treat '-' as a normal character
            adj[start]['-'].insert(end);
            
            return {start, end};
        }
        
        char range_start = adj[start].begin()->first;
        char range_end = adj[char_res.first].begin()->first;
        
        if (range_start > range_end) { // treat as normal characters, for example: z-a as 'z' '-' 'a'
            adj[start][range_start].insert(end);
            adj[start]['-'].insert(end);
            adj[start][range_end].insert(end);
        } else {
            for (int c = range_start; c <= range_end; c++)
                adj[start][c].insert(end);
        }

        return {start, end};
    }
    
    return lvalue; // <empty> case
}

pair<int, int> parse_char() {
    const string meta_characters = "[]\\.^$*+?{}|()";
    if (peek() == '\\') {
        match('\\');

        int c = match_one_of(meta_characters);

        int start = make_node();
        int end = make_node();
        adj[start][c].insert(end);

        return {start, end};
    } else if (0 <= peek() && peek() < 256 && meta_characters.find(peek()) == string::npos) {
        int c = match_none_of(meta_characters);

        int start = make_node();
        int end = make_node();
        adj[start][c].insert(end);

        return {start, end};
    }

    // peek() is not meant to be parsed here
    return {-1, -1};
}

int main() {
    adj.push_back(unordered_map<int, set<int>>()); // 0 is the dead state

    cin >> pattern;

    parse_RE();

    assert(i == int(pattern.size()));

    return 0;
}