# dumb_regex

**dumb_regex** is a regular expression engine implemented in C++. It supports a basic subset of regex functionality and uses classic algorithms to convert regex patterns into a minimized DFA (Deterministic Finite Automaton). This project is not optimized for performance or edge cases. It is built purely for learning, experimenting, and understanding how regex engines work under the hood.

## Quick Start

```bash
g++ main.cpp regex.cpp -o dumb_regex
./dumb_regex
```

> Note: There is no user interface or interactive frontend. The `main.cpp` file contains sample usage code that you can modify to test different regex patterns and input strings.

## Current Limitations

- Only supports matching some input against a pattern
- No support for word boundary character classes `\b`, `\B`
- No support for look-ahead or look-behind assertions
- No support for backreferences

## References
- https://regex101.com/
- https://en.wikipedia.org/wiki/Regular_expression
- https://en.wikipedia.org/wiki/Thompson%27s_construction
- https://www.cs.sfu.ca/~cameron/Teaching/384/99-3/regexp-plg.html
- https://web.archive.org/web/20090129224504/http://faqts.com/knowledge_base/view.phtml/aid/25718/fid/200
- https://en.wikipedia.org/wiki/Left_recursion
- https://github.com/lotabout/write-a-C-interpreter
- https://swtch.com/~rsc/regexp/regexp1.html
- https://en.wikipedia.org/wiki/Powerset_construction
- https://en.wikipedia.org/wiki/DFA_minimization
- https://web.cecs.pdx.edu/~harry/compilers/slides/LexicalPart4.pdf
- https://graphviz.org/Gallery/directed/fsm.html

## License

This project is released under the MIT License. See [LICENSE](LICENSE) for details.
