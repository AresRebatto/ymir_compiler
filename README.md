# Ymir Compiler
It is a compiler for a very simple C-like language designed for the simple purpose of being able to write the compiler: it is in fact a project of an educational nature
## Install
To install, copy and paste the following code
```
	curl -L https://github.com/AresRebatto/ymir_compiler/setup/install.sh | bash
```
## Uninstall
## How it works
The compiler follows the steps of lexing and parsing user-supplied source code and then converting it to native Rust using macros.

The grammar is defined in `src/components/grammar.yaml` with
BNF notation
## The language that is defined
The defined language is very simple that is structured with the following trivial Tokens to structure a language to write basic 
algorithms(copy of TokenKind enum):
- `Int(i32)`
- `Equal`(=)
- `Minus`
- `Plus`
- `Semicolon`(;)
- `Type(String)`
- `Identifier(String)`: is the name of a created variable
## Dependencies
