#[allow(nonstandard_style)]
pub enum Class {
    Documentation_Inner_Block,
    Documentation_Inner_Line,
    Documentation_Outer_Block,
    Documentation_Outer_Line,
    Identifier_Plain,
    Identifier_Raw,
    Lifetime,
    Literal_ByteString_Suffixed,
    Literal_ByteString_Unsuffixed,
    Literal_Byte_Suffixed,
    Literal_Byte_Unsuffixed,
    Literal_Character_Suffixed,
    Literal_Character_Unsuffixed,
    Literal_Float_Suffixed,
    Literal_Float_Unsuffixed,
    Literal_Integer_Binary_Suffixed,
    Literal_Integer_Binary_Unsuffixed,
    Literal_Integer_Decimal_Suffixed,
    Literal_Integer_Decimal_Unsuffixed,
    Literal_Integer_Hexadecimal_Suffixed,
    Literal_Integer_Hexadecimal_Unsuffixed,
    Literal_Integer_Octal_Suffixed,
    Literal_Integer_Octal_Unsuffixed,
    Literal_RawByteString_Suffixed,
    Literal_RawByteString_Unsuffixed,
    Literal_RawString_Suffixed,
    Literal_RawString_Unsuffixed,
    Literal_String_Suffixed,
    Literal_String_Unsuffixed,
    Punctuation_Ampersand,
    Punctuation_At,
    Punctuation_Bar,
    Punctuation_Brace_Close,
    Punctuation_Brace_Open,
    Punctuation_Bracket_Close,
    Punctuation_Bracket_Open,
    Punctuation_Circumflex,
    Punctuation_Colon,
    Punctuation_Comma,
    Punctuation_Dollar,
    Punctuation_Dot,
    Punctuation_Equal,
    Punctuation_Exclamation,
    Punctuation_Greater,
    Punctuation_Less,
    Punctuation_Minus,
    Punctuation_Parenthesis_Close,
    Punctuation_Parenthesis_Open,
    Punctuation_Percent,
    Punctuation_Plus,
    Punctuation_Pound,
    Punctuation_Question,
    Punctuation_Semicolon,
    Punctuation_Slash,
    Punctuation_Star,
    Punctuation_Tilde,
    Trivia_Comment_Block,
    Trivia_Comment_Line,
    Trivia_Whitespace,
}

pub struct Token {
    class: Class,
    length: usize,
}

pub trait Cooker {
    fn cook_all(raw: &[crate::raw::Token]) -> Result<Vec<Token>, ()>;
}
