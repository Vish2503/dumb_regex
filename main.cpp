#include "regex.h"

#include <iostream>
#include <vector>

using namespace std;

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