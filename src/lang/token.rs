
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

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Modulus,

    // Various types (Whooo a lot)
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    LongLong(i128),
    ULongLong(u128),
    Float(f32),
    Double(f64),

    String(String),
    Boolean(bool)
}