
use std::{collections::VecDeque, fmt::{Debug, Display}};

// All imported modules
use super::{consts, errors::illegal_char_error::IllegalCharError, token::Token};

// Tokenize an identifier
fn tokenize_identifier(chars: &mut VecDeque<char>, c: char) -> Token {
    // Setup the data for the token
    let mut token_data = String::from(c);

    // Loop through characters until the identifier can't be continued
    while let Some(c2) = chars.pop_front() {
        if c2.is_alphanumeric() {
            token_data.push(c2);
        } else {
            chars.push_front(c2);
            break;
        }
    }

    // Return the token
    if consts::KEYWORDS.contains(&token_data.as_str()) {
        Token::Keyword(token_data)
    } else {
        Token::Identifier(token_data)
    }
}

// Tokenize a number
fn tokenize_number(chars: &mut VecDeque<char>, c: char) -> Result<Token, IllegalCharError> {

    // Variables
    let mut dot = false;
    let mut num_str = String::from(c);

    // While there are still characters left
    while let Some(c2) = chars.pop_front() {

        // If it's a dot, mark the dot or throw an error
        if c2 == '.' {
            match dot {
                true => return Err(IllegalCharError('.')),
                false => dot = true
            }
        }

        // If it's a valid character then push it, else put it back and break
        if c2.is_numeric() || c2 == '.' {
            num_str.push(c2);
        } else {
            chars.push_front(c2);
            break;
        }
    }

    // Return the correct type
    if dot {
        let num = num_str.parse::<f32>().unwrap();
        Ok(Token::Float(num))
    } else {
        let num = num_str.parse::<i32>().unwrap();
        Ok(Token::Int(num))
    }
}

// Tokenize a string
pub fn tokenize(text: String) -> Result<Vec<Token>, impl Display + Debug> {

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
            let tok = tokenize_number(&mut chars, c)?;
            tokens.push(tok);
        
        // Newlines and semicolons
        } else if c == ';' || c == '\n' {

            // Create the token and push it
            tokens.push(Token::Newline)
        
        // Nothing happens on whitespace
        } else if c.is_whitespace() {

        // If it's a completely unexpected character
        } else {
            return Err(IllegalCharError(c));
        }
    }

    // Return the tokens
    Ok::<Vec<Token>, _>(tokens)
}