
use std::collections::VecDeque;

// All imported modules
use super::{consts, token::Token};

// Tokenize an identifier
fn tokenize_identifier(chars: &mut VecDeque<char>, c: char) -> Token {
    // Setup the data for the token
    let mut token_data = String::from(c);

    // Loop through characters until the identifier can't be continued
    // Would be cleaner but you can't use a while let in a while let
    let alphanumeric_chars: VecDeque<_> = chars.iter()
        .take_while(|c| c.is_alphanumeric())
        .cloned()
        .collect();

    chars.drain(0..alphanumeric_chars.len());
    token_data.extend(alphanumeric_chars);

    // Return the token
    if consts::KEYWORDS.contains(&token_data.as_str()) {
        Token::Keyword(token_data)
    } else {
        Token::Identifier(token_data)
    }
}

// Tokenize a number
fn tokenize_number(chars: &mut VecDeque<char>, c: char) -> Token {
    todo!()
}

// Tokenize a string
pub fn tokenize(text: String) -> Vec<Token> {

    // Return tokens
    let mut tokens: Vec<Token> = Vec::new();

    // Get all the characters
    let mut chars: VecDeque<char> = VecDeque::from_iter(text.chars());

    // For all the characters
    while let Some(c) = chars.pop_front() {

        // If the character is in the alphabet
        if c.is_alphabetic() {

            // Create the token and push it
            let tok = tokenize_identifier(&mut chars, c);
            tokens.push(tok);

        // If the character is a digit
        } else if c.is_digit(10) {
            
            // Create the token and push it
            let tok = tokenize_number(&mut chars, c);
            tokens.push(tok);
        }
    }

    // Return the tokens
    tokens
}