# Plain Programming Language

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)

Plain is a minimalist, English-like programming language designed to make programming feel like natural conversation. It combines the power of traditional programming with intuitive, human-readable syntax.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Language Syntax](#language-syntax)
  - [Variables and Assignment](#variables-and-assignment)
  - [Arithmetic Operations](#arithmetic-operations)
  - [Output and Display](#output-and-display)
  - [Conditional Statements](#conditional-statements)
  - [Loops](#loops)
  - [Sequences and Flow Control](#sequences-and-flow-control)
  - [Pronouns and Context](#pronouns-and-context)
- [Architecture](#architecture)
- [Implementation Details](#implementation-details)
- [CLI Interface](#cli-interface)
- [Examples](#examples)
- [API Reference](#api-reference)
- [Development](#development)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## Overview

Plain reimagines programming by using natural English sentences instead of cryptic symbols and keywords. Instead of writing:

```javascript
let distance = 5;
distance += 18;
console.log(distance);
```

You can write:

```plain
set the distance to 5.
add 18 to the distance then display it.
```

The language maintains full programming power while being accessible to beginners and readable like plain English.

## Features

- ✅ **Natural English Syntax**: Write code using everyday language patterns
- ✅ **Pronoun Support**: Use "it" to refer to previous results
- ✅ **Flexible Statements**: Support for sequences with "then"
- ✅ **Mathematical Operations**: Addition, subtraction, and comparisons
- ✅ **Conditional Logic**: If-then statements with natural syntax
- ✅ **Loop Constructs**: Count-based iteration
- ✅ **Variable Management**: Simple variable declaration and manipulation
- ✅ **Output Handling**: Display results with natural phrasing
- ✅ **Interactive REPL**: Command-line interface for experimentation
- ✅ **Extensible Architecture**: Clean separation of concerns for future features

## Installation

### Prerequisites

- **Rust**: Version 1.70 or later
- **Cargo**: Rust's package manager (included with Rust)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/plain-lang.git
cd plain-lang

# Build the project
cargo build --release

# Run tests (optional)
cargo test
```

The compiled binary will be available at `target/release/plain-lang`.

### Direct Usage

After building, you can run Plain programs directly:

```bash
# Run a Plain source file
./target/release/plain-lang run examples/demo.plain

# Start the interactive REPL
./target/release/plain-lang
```

## Quick Start

Create a file called `hello.plain`:

```plain
set the greeting to "Hello World".
show on screen the greeting.
```

Run it:

```bash
plain-lang run hello.plain
```

Expected output:
```
Hello World
```

## Language Syntax

### Variables and Assignment

**Basic Assignment:**
```plain
set the variable_name to 42.
set my_name to "Alice".
```

**With Articles:**
```plain
set the distance to 100.
set the message to "Welcome".
```

Variables can hold integers or strings and are case-sensitive.

### Arithmetic Operations

**Addition:**
```plain
set the score to 10.
add 5 to the score.
add 3 to score then display it.
```

**Subtraction:**
```plain
set the temperature to 72.
subtract 10 from the temperature.
```

**Result Tracking:**
All arithmetic operations automatically update the internal "last result" for pronoun usage.

### Output and Display

**Basic Display:**
```plain
set the message to "Hello".
show on screen the message.
display the message.
```

**Pronoun Usage:**
```plain
add 5 to the counter then display it.
```

**Flexible Output:**
```plain
show on screen the result.
display the current_value.
```

### Conditional Statements

**Simple Conditions:**
```plain
set the age to 25.
if age is greater than 18 then show on screen "Adult".
```

**With Articles:**
```plain
if the temperature is less than 0 then display "Freezing".
```

**Comparison Operators:**
- `is greater than`
- `is less than`
- `is equal to`

### Loops

**Count-based Loops:**
```plain
count to 5 and when you are done display "Done".
set the counter to 0.
count to 10 and when you are done show on screen the counter.
```

The loop executes the body statement the specified number of times.

### Sequences and Flow Control

**Statement Chaining:**
```plain
set the value to 10 then add 5 to value then display it.
```

**Complex Sequences:**
```plain
set the score to 0.
add 10 to the score then display it.
if score is greater than 5 then add 5 to score then display it.
```

### Pronouns and Context

**Automatic Context Tracking:**
```plain
set the distance to 100.
add 50 to the distance.
display it.
```

In this example, "it" refers to the result of the addition (150).

**Pronoun Rules:**
- "it" always refers to the most recent computed result
- Pronouns work across all operations that produce values
- Context is maintained throughout program execution

## Architecture

### Core Components

```
plain-lang/
├── src/
│   ├── main.rs       # CLI entry point
│   ├── lexer.rs      # Tokenization (logos-based)
│   ├── parser.rs     # Recursive descent parsing
│   ├── ast.rs        # Abstract Syntax Tree definitions
│   ├── runtime.rs    # Execution engine
│   ├── repl.rs       # Interactive REPL
│   └── lib.rs        # Module declarations
├── examples/         # Sample programs
└── Cargo.toml        # Dependencies and metadata
```

### Execution Pipeline

1. **Lexical Analysis**: Source text → tokens
2. **Parsing**: Tokens → AST
3. **Type Checking**: Semantic validation (stub for future)
4. **Code Generation**: AST → executable form (interpreter)
5. **Execution**: Runtime evaluation with state management

### Key Design Decisions

- **Interpreter Architecture**: Tree-walking interpreter for simplicity and debugging
- **Context Tracking**: `last_value` system for natural pronoun support
- **Flexible Parsing**: Extensive optional tokens for natural language variation
- **Error Handling**: Descriptive error messages for debugging

## Implementation Details

### Lexer (lexer.rs)

- Uses the `logos` crate for efficient tokenization
- Case-insensitive keywords with regex patterns
- Comprehensive token set covering English-like constructs

### Parser (parser.rs)

- Recursive descent parser with manual precedence handling
- Extensive support for optional tokens ("the", articles, etc.)
- Context-aware parsing with position tracking
- Robust error reporting with position information

### Runtime (runtime.rs)

- Variable storage using `HashMap<String, Value>`
- Result caching with `last_value` for pronoun support
- Tree-walking evaluation of AST nodes
- Type-safe operations with explicit error handling

### AST (ast.rs)

```rust
pub enum Stmt {
    Set(String, Expr),
    Add(Expr, String),
    Sub(Expr, String),
    Show(Expr),
    If(Expr, Box<Stmt>),
    Seq(Box<Stmt>, Box<Stmt>),
    Loop(Expr, Box<Stmt>),
}

pub enum Expr {
    Int(i64),
    Str(String),
    Var(String),
    LastValue,  // For pronouns
    Gt(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
}
```

## CLI Interface

### Commands

```bash
plain-lang run <file>    # Execute a Plain source file
plain-lang               # Start interactive REPL
plain-lang --help        # Display help information
```

### REPL Mode

```
$ plain-lang
Plain> set the value to 42.
Plain> display it.
42
Plain> add 8 to value then display it.
50
```

### File Execution

```bash
$ plain-lang run examples/demo.plain
23
23
...
```

## Examples

### Basic Arithmetic
```plain
set the x to 10.
add 5 to x.
display it.
```

### Conditional Logic
```plain
set the temperature to 75.
if temperature is greater than 70 then display "Warm".
```

### Loops with Context
```plain
set the counter to 0.
count to 5 and when you are done display the counter.
```

### Complex Sequences
```plain
set the score to 100.
add 25 to score then display it.
if score is greater than 120 then show on screen "High Score".
```

## API Reference

### Core Functions

#### `parse(input: &str) -> Result<Stmt, String>`
Parses Plain source code into an AST statement.

#### `Runtime::new() -> Runtime`
Creates a new execution environment.

#### `Runtime::exec_stmt(&mut self, stmt: &Stmt) -> Result<(), String>`
Executes an AST statement in the current environment.

### Data Structures

#### `Runtime`
- `vars: HashMap<String, Value>` - Variable storage
- `last_value: Option<Value>` - Most recent result for pronouns

#### `Value` enum
- `Int(i64)` - Integer values
- `Str(String)` - String values

## Development

### Project Structure

```
plain-lang/
├── src/
│   ├── main.rs          # CLI and main entry point
│   ├── lexer.rs         # Tokenization logic
│   ├── parser.rs        # Language grammar and parsing
│   ├── ast.rs           # Abstract syntax tree definitions
│   ├── runtime.rs       # Execution environment
│   ├── repl.rs          # Interactive shell
│   ├── codegen.rs       # Code generation (future JIT)
│   └── typecheck.rs     # Type checking (future features)
├── examples/            # Sample Plain programs
├── tests/              # Unit and integration tests
└── Cargo.toml          # Project configuration
```

### Adding New Features

1. **Lexer**: Add new tokens to `Token` enum in `lexer.rs`
2. **Parser**: Extend `parse_stmt` and `parse_expr` functions
3. **AST**: Add new variants to `Stmt` and `Expr` enums
4. **Runtime**: Implement execution logic for new features
5. **Tests**: Add test cases for new functionality

### Testing Framework

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_parser

# Run with verbose output
cargo test -- --nocapture
```

## Testing

### Test Categories

- **Unit Tests**: Individual component testing
- **Integration Tests**: End-to-end program execution
- **Parser Tests**: Grammar and syntax validation
- **Runtime Tests**: Execution correctness

### Writing Tests

```rust
#[test]
fn test_basic_assignment() {
    let input = "set the value to 42.";
    let result = parse(input);
    assert!(result.is_ok());

    let mut runtime = Runtime::new();
    let stmt = result.unwrap();
    assert!(runtime.exec_stmt(&stmt).is_ok());
    assert_eq!(runtime.vars.get("value"), Some(&Value::Int(42)));
}
```

## Contributing

We welcome contributions! Please follow these guidelines:

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/new-feature`
3. Commit your changes: `git commit -am 'Add new feature'`
4. Push to the branch: `git push origin feature/new-feature`
5. Submit a pull request

### Code Standards

- **Rust Style**: Follow standard Rust formatting (`cargo fmt`)
- **Documentation**: Add doc comments for public APIs
- **Testing**: Include tests for new functionality
- **Error Handling**: Use descriptive error messages
- **Performance**: Consider efficiency in algorithm design

### Areas for Contribution

- **New Language Features**: Control flow, functions, data structures
- **Performance Optimization**: JIT compilation, caching
- **Error Handling**: Better error messages and recovery
- **Tooling**: IDE support, debuggers, formatters
- **Documentation**: Tutorials, examples, language specification

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### MIT License Summary

Copyright (c) 2025 Studio Platforms

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

---

## Acknowledgments

- **Rust Community**: For the excellent language and ecosystem
- **Logos**: Efficient tokenization library
- **Chumsky**: Parser combinator framework (inspiration)
- **Open Source Community**: For inspiration and tools

## Future Roadmap

- [ ] **Function Definitions**: `define function_name as ...`
- [ ] **Data Structures**: Arrays and objects
- [ ] **File I/O**: Reading and writing files
- [ ] **Modules**: Code organization and imports
- [ ] **JIT Compilation**: Performance optimization with Cranelift
- [ ] **Debugger**: Step-through execution and breakpoints
- [ ] **Package Manager**: Dependency management
- [ ] **Web Integration**: Browser-based execution

---

*Plain: Making programming as natural as conversation.* 
