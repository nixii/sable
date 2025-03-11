
// All imported modules
use super::token::Token;

// Tokenize a string
pub fn tokenize(text: String) -> Vec<Token> {

    // Return tokens
    let mut tokens: Vec<Token> = Vec::new();

    // Get all the characters
    let mut chars = text.chars();

    // For all the characters
    while let Some(c) = chars.nth(0) {

        // If the character is in the alphabet
        if c.is_alphabetic() {

            // Setup the data for the token
            let mut token_data = String::from(c);

            // Loop through characters until the identifier can't be continued
            // Would be cleaner but you can't use a while let in a while let
            loop {
                let c2 = chars.nth(0);
                match c2 {
                    Some(x) => {
                        if x.is_alphanumeric() {
                            token_data.push(x);
                        } else {
                            break;
                        }
                    },
                    None => break
                }
            }

            // Push the identifier in
            tokens.push(Token::Identifier(token_data));
        } else if c.is_digit(10) {

        }
    }

    // Return the tokens
    tokens
}