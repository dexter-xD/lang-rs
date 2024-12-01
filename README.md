# Custom Programming Language - Tokenizer

This project is the foundation for a simple programming language. It currently includes a **tokenizer**, which breaks down the source code into meaningful tokens.

## Example Code in This Language

```plaintext
a = 7       // Assign the value 7 to variable `a`
b = 2       // Assign the value 2 to variable `b`
c = a * b   // Multiply `a` and `b`, and assign the result to `c`
print(c)    // Output the value of `c`
```

## Features
- Recognizes numbers, identifiers (variables), and basic operators (`+`, `-`, `*`, `/`, `=`).
- Tokenizes source code for further compilation or interpretation.

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/) (latest stable version)

### Clone the Repository
```bash
git clone https://github.com/dexter-xD/lang-rs
cd lang-rs
```

### Run the Program
1. Open the terminal in the project directory.
2. Use the following command to run the tokenizer with example input:
   ```bash
   cargo run
   ```

### Modify Input Code
You can edit the source code example in the `main` function of the `src/main.rs` file to test different inputs.

## Example Output
For the input:
```plaintext
a = 7
b = 2
c = a * b
print(c)
```

The tokenizer generates:
```plaintext
Token { token_type: Identifier, lexeme: "a" }
Token { token_type: Equal, lexeme: "=" }
Token { token_type: NumberLiteral, lexeme: "7" }
Token { token_type: Newline, lexeme: "\n" }
Token { token_type: Identifier, lexeme: "b" }
Token { token_type: Equal, lexeme: "=" }
Token { token_type: NumberLiteral, lexeme: "2" }
...
```