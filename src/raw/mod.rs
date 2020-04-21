#[cfg(feature = "logos")]
pub mod logos_impl;
pub mod reference_impl;

#[allow(nonstandard_style)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Class {
    line_comment,
    whitespace,
    identifier,
    raw_identifier,
    identifier_fragment,
    lifetime,
    binary_integer,
    binary_float,
    octal_integer,
    octal_float,
    hexadecimal_integer,
    decimal_integer,
    decimal_float,
    character,
    byte,
    string,
    byte_string,
    exclamation,
    pound,
    dollar,
    percent,
    ampersand,
    open_parenthesis,
    close_parenthesis,
    star,
    plus,
    comma,
    minus,
    dot,
    slash,
    colon,
    semicolon,
    less,
    equal,
    greater,
    question,
    at,
    open_bracket,
    close_bracket,
    circumflex,
    open_brace,
    bar,
    close_brace,
    tilde,
    block_comment,
    raw_string,
    raw_byte_string,
}
pub use Class::*;

impl Class {
    pub const COUNT: usize = raw_byte_string as usize + 1;
    pub const ALL: [Class; Class::COUNT] = [
        line_comment,
        whitespace,
        identifier,
        raw_identifier,
        identifier_fragment,
        lifetime,
        binary_integer,
        binary_float,
        octal_integer,
        octal_float,
        hexadecimal_integer,
        decimal_integer,
        decimal_float,
        character,
        byte,
        string,
        byte_string,
        exclamation,
        pound,
        dollar,
        percent,
        ampersand,
        open_parenthesis,
        close_parenthesis,
        star,
        plus,
        comma,
        minus,
        dot,
        slash,
        colon,
        semicolon,
        less,
        equal,
        greater,
        question,
        at,
        open_bracket,
        close_bracket,
        circumflex,
        open_brace,
        bar,
        close_brace,
        tilde,
        block_comment,
        raw_string,
        raw_byte_string,
    ];
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Token {
    pub class: Class,
    pub length: usize,
}

pub trait Lexer {
    fn lex(s: &str) -> Result<Token, ()>;
    fn lex_all(s: &str) -> Result<Vec<Token>, ()> {
        let mut s = s;
        std::iter::from_fn(|| {
            if s.is_empty() {
                return None;
            } else {
                match Self::lex(s) {
                    Ok(token) => {
                        s = &s[token.length..];
                        Some(Ok(token))
                    }
                    Err(()) => {
                        s = "";
                        Some(Err(()))
                    }
                }
            }
        })
        .collect()
    }
}
