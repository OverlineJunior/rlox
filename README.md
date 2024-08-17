Rlox is an almost one-to-one copy of the Jlox interpreter from [Crafting Interpreters](https://craftinginterpreters.com/), but written in Rust.

# How it works


Because Rlox is a tree-walking interpreter, it takes a shortcut, following the path of scanning -> parsing -> interpreting, skipping over analysis and code generation.

![Map](https://craftinginterpreters.com/image/a-map-of-the-territory/mountain.png)

Process | Input | Output
-- | -- | -- |
Scanning | Source code | Tokens
Parsing | Tokens | Syntax Tree
Interpreting | Syntax Tree | State & Effects

## Scanner



## Parser



## Interpreter



# Progress

- [x] Scanning
- [x] Representing Code
- [x] Parsing
- [x] Evaluating Expressions
- [x] Statements and State
- [ ] Control Flow
- [ ] Functions
- [ ] Resolving and Binding
- [ ] Classes
- [ ] Inheritance
