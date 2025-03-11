
// All imported modules
use super::token::Token;

// Tokenize a string
pub fn tokenize(text: String) -> Vec<Token> {

    // Return tokens
    let tokens: Vec<Token> = Vec::new();

    // Get all the characters
    let mut chars = text.chars();

    // For all the characters
    while let Some(c) = chars.nth(0) {

        if c.is_alphabetic() {

        } else if c.is_digit(10) {

        }
    }

    // Return the tokens
    tokens
}