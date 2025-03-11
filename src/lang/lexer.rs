
use std::collections::VecDeque;

// All imported modules
use super::token::Token;

// Tokenize an identifier
fn tokenize_identifier(chars: &mut VecDeque<char>, c: char) -> Token {
    // Setup the data for the token
    let mut token_data = String::from(c);

    // Loop through characters until the identifier can't be continued
    // Would be cleaner but you can't use a while let in a while let
    // loop {
    //     let c2 = chars.pop_front();
    //     match c2 {
    //         Some(x) => {
    //             if x.is_alphanumeric() {
    //                 token_data.push(x);
    //             } else {
    //                 break;
    //             }
    //         },
    //         None => break
    //     }
    // }
    let alphanumeric_chars: VecDeque<_> = chars.iter()
        .take_while(|c| c.is_alphanumeric())
        .cloned()
        .collect();

    chars.drain(0..alphanumeric_chars.len());
    token_data.extend(alphanumeric_chars);

    // Return the token
    Token::Identifier(token_data)
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
            let mut num_str = String::from(c);
            let mut dot = false;
            
            loop {
                let c2 = chars.pop_front();
                match c2 {
                    Some(x) => {
                        if x.is_digit(10) || x == '.' {
                            if x == '.' {
                                dot = true;
                            }
                            num_str.push(x);
                        } else {
                            break;
                        }
                    },
                    None => break
                }
            }

            if dot {
                
            }
        }
    }

    // Return the tokens
    tokens
}