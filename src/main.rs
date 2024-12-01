// Welcome!
// This language allows basic mathematical operations and simple print functionality.
// Example program in this language:
// ------------------------------------
// a = 7       // Assign the value 7 to variable `a`
// b = 2       // Assign the value 2 to variable `b`
// c = a * b   // Multiply `a` and `b`, and assign the result to `c`
// print(c)    // Output the value of `c`
// ------------------------------------
// In this video, we'll focus on building the **tokenizer**,
// the first step of the compiler that breaks the input into meaningful tokens.

// Define the types of tokens our tokenizer will identify.
#[derive(Debug, PartialEq)]
pub enum TokenType {
    NumberLiteral, // Example: 2, 3, 17
    Identifier,    // Example: a, b, c, aComplicatedVariableName
    Equal,         // '=' operator
    Plus,          // '+' operator
    Minus,         // '-' operator
    Star,          // '*' operator
    Slash,         // '/' operator
    LeftParen,     // '(' symbol
    RightParen,    // ')' symbol
    Newline,       // '\n' to mark a new line
}

// Make the `TokenType` variants easier to use without fully qualifying them.
use TokenType::*;

// A `Token` represents a meaningful unit in the source code, e.g., numbers, variables, operators.
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType, // The type of token
    pub lexeme: String,        // The actual string representation, e.g., "123" for a number
}

// Main function to convert raw source code into a list of tokens.
pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut position = 0; // Keeps track of the current position in the source code.
    let mut result = Vec::new(); // Stores the tokens we generate.

    // Process the source code character by character.
    while position < source_code.len() {
        let current_char = source_code.chars().nth(position).unwrap();

        match current_char {
            // Match single-character operators/symbols and create tokens for them.
            '=' => result.push(Token {
                token_type: Equal,
                lexeme: "=".to_string(),
            }),
            '*' => result.push(Token {
                token_type: Star,
                lexeme: "*".to_string(),
            }),
            '/' => result.push(Token {
                token_type: Slash,
                lexeme: "/".to_string(),
            }),
            '+' => result.push(Token {
                token_type: Plus,
                lexeme: "+".to_string(),
            }),
            '-' => result.push(Token {
                token_type: Minus,
                lexeme: "-".to_string(),
            }),
            '(' => result.push(Token {
                token_type: LeftParen,
                lexeme: "(".to_string(),
            }),
            ')' => result.push(Token {
                token_type: RightParen,
                lexeme: ")".to_string(),
            }),
            '\n' => result.push(Token {
                token_type: Newline,
                lexeme: "\n".to_string(),
            }),

            // Handle number literals (e.g., "123").
            x if x.is_digit(10) => {
                let mut number_lexeme = x.to_string(); // Start building the number string.
                position += 1;

                // Keep adding digits to the number until a non-digit is encountered.
                while position < source_code.len() {
                    let next_char = source_code.chars().nth(position).unwrap();

                    // Stop if we encounter a space, closing parenthesis, or newline.
                    if next_char == ' ' || next_char == ')' || next_char == '\n' {
                        break;
                    }

                    if next_char.is_digit(10) {
                        number_lexeme.push(next_char); // Append digit to the number.
                    } else {
                        panic!("Invalid character in number: '{}'", next_char); // Error on invalid characters.
                    }

                    position += 1;
                }

                result.push(Token {
                    token_type: NumberLiteral,
                    lexeme: number_lexeme,
                });
                continue; // Skip incrementing position since it's already handled.
            }

            // Skip spaces as they are not meaningful tokens.
            ' ' => {}

            // Handle identifiers (e.g., variable names like "print" or "a").
            c => {
                let mut lexeme = c.to_string(); // Start building the identifier string.
                position += 1;

                // Keep adding valid characters to the identifier.
                while position < source_code.len() {
                    let next_char = source_code.chars().nth(position).unwrap();

                    if !is_valid_identifier_char(next_char) {
                        break; // Stop if the character is not valid for identifiers.
                    }

                    lexeme.push(next_char);
                    position += 1;
                }

                result.push(Token {
                    token_type: Identifier,
                    lexeme,
                });
                continue;
            }
        }

        position += 1; // Move to the next character.
    }

    return result;
}

// Helper function to check if a character is valid in an identifier (letters, digits, underscores).
fn is_valid_identifier_char(ch: char) -> bool {
    return ch.is_alphanumeric() || ch == '_';
}

fn main() {
    let src = "a = 123
                    print(a)"; // Example source code
    let tokens = tokenize(src); // Tokenize the source code.

    // Print the generated tokens for debugging or inspection.
    for token in tokens {
        println!("{:?}", token);
    }
}
