
// Simple token enum
#[derive(Debug)]
pub enum Token {

    // Text stuff
    Keyword(String),
    Identifier(String),
    
    // Encapsulators
    LeftCurlBrace,
    RightCurlBrace,

    LeftParen,
    RightParen,

    DoubleQuote,
    SingleQuote,

    // Random data stuff
    Newline,
    Semicolon,

    // Number types (Whooo a lot)
    Integer(i32),
    UInteger(u32),
    Float(f32),
}