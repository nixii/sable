
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

    // Random data stuff
    Newline,
    Semicolon,

    // Various types (Whooo a lot)
    Integer(i32),
    UInteger(u32),
    Float(f32),
    String(String),
    Boolean(bool)
}