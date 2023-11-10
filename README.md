# Various projects

projects done in Rust to learn the language.

TODO List

- [x] cat
- [x] ls
- [ ] tinysh
- [ ] calc

# Building

To build the projects do

```bash
cargo build --release
```

## Cat

Cat: I hope you know what cat is

```bash 
./target/release/cat <file>
```

### Usage

![cat](assets/cat.png)

## LS

LS: a minimalist implementation of ls

```bash 
./target/release/ls <dir>
```

### Usage

You can just do

```bash 
./target/release/ls <dir>
```

which produces

![](assets/ls_vanilla.png)

However this one has _options_

### Options

#### Colour

You can add _colours_ to the output with the `--color` or `-c` option
![](assets/ls_colors.png)

#### Sort

You can sort alphabetically the output with the `--sort` or `-s` option
![](assets/ls_sort.png)

#### Both

And you can combine them to have a _beautiful_ **sorted** output
![](assets/ls_colors_sort.png)

## Tinysh

Tinysh: a minimalistic for educational purpose only shell

TODO List

- [ ] Add built in
    - [ ] ls
    - [ ] cat
    - [ ] cd
    - [ ] calc
- [ ] Launch command from the shell in the user PATH
- [ ] Add new folders to path
- [ ] Add redirection in files
    - [ ] \>
    - [ ] \<
    - [ ] \>>
    - [ ] \<<
- [ ] Add piping
- [ ] Customize the prompt

## Calc

Calc: a minimalistic calculator built for educational purpose only.

TODO List

- [X] Lexing of basic operations
  - [X] Lexing operators
  - [X] Lexing lpar,rpar,quote
  - [X] Lexing int
  - [X] Lexing floats
  - [X] Lexing strings
- [ ] Parsing of basic operations
  - [ ] sum operation
  - [ ] minus operation
  - [ ] multiplication operation
  - [ ] division operation
  - [ ] mathematical priority
  - [ ] parenthesis support
  - [ ] associativity support
  - [ ] hidden multiplication support
- [X] Application REPL
  - [X] Add the REPL
    - [X] basic REPL
    - [X] colour message
    - [X] colourised prompt
  - [X] Add colour
- [ ] Config
  - [ ] Config parser
  - [ ] Config colours
  - [ ] Config prompt
- [ ] Add support for functions
  - [ ] exp
  - [ ] ln
  - [ ] log base a
  - [ ] cos/sin/tan 
  - [ ] cosh/sinh/tanh
- [ ] Defining your own functions
- [ ] Add RPN mode