
// Imports
use std::fmt::Display;

// Create the struct
#[derive(Debug)]
pub struct IllegalCharError(pub char);

// Implement display for the error
impl Display for IllegalCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Illegal character: {}", self.0)
    }
}