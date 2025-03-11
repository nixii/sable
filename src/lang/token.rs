
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
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    Integer128(i128),
    UInteger8(u8),
    UInteger16(u16),
    UInteger32(u32),
    UInteger64(u64),
    UInteger128(u128),
    Float32(f32),
    Float64(f64),
}