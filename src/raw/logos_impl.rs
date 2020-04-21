use logos::Logos;

#[allow(nonstandard_style)]
#[derive(Logos, Debug, Eq, PartialEq)]
enum TokenKind {
    #[regex(r"//[^\n]*")]
    line_comment,
    #[regex(r"\p{Pattern_White_Space}+")]
    whitespace,
    #[regex(r"[_\p{XID_Start}]\p{XID_Continue}*")]
    identifier,
    #[regex(r"r#[_\p{XID_Start}]\p{XID_Continue}*")]
    raw_identifier,
    #[regex(r"[\p{XID_Continue}--_0-9\p{XID_Start}]\p{XID_Continue}*")]
    identifier_fragment,
    #[regex(r"'[_\p{XID_Start}]\p{XID_Continue}*")]
    lifetime,
    #[regex(r"0b[_0-9]*")]
    binary_integer,
    #[regex(r"0b[_0-9]*[eE][+-]?[_0-9]*")]
    binary_float,
    #[regex(r"0o[_0-9]*")]
    octal_integer,
    #[regex(r"0o[_0-9]*[eE][+-]?[_0-9]*")]
    octal_float,
    #[regex(r"0x[_0-9a-fA-F]*")]
    hexadecimal_integer,
    #[regex(r"[0-9][_0-9]*")]
    decimal_integer,
    #[regex(r"[0-9][_0-9]*[eE][+-]?[_0-9]*")]
    decimal_float,
    #[regex(r#"'(?:[^\t\n\r\\']|\\['"]|\\[nrt\\0]|\\x[0-7][0-9a-fA-F]|\\u\{(?:10|[0-9])[0-9a-fA-F]?[0-9a-fA-F]?[0-9a-fA-F]?[0-9a-fA-F]?\})'"#)]
    character,
    #[regex(r#"'(?:[[:ascii:]--\t\n\r\\']|\\['"]|\\[nrt\\0]|\\x[0-9a-fA-F][0-9a-fA-F])'"#)]
    byte,
    #[regex(r#""(?:[^\t\n\r\\"]|\r\n|\\\n\p{Pattern_White_Space}*|\\['"]|\\[nrt\\0]|\\x[0-7][0-9a-fA-F]|\\u\{(?:10|[0-9])[0-9a-fA-F]?[0-9a-fA-F]?[0-9a-fA-F]?[0-9a-fA-F]?\})*""#)]
    string,
    #[regex(r#"b"(?:[[:ascii:]--\r\\"]|\r\n|\\\n\p{Pattern_White_Space}*|\\['"]|\\[nrt\\0]|\\x[0-9a-fA-F][0-9a-fA-F])*""#)]
    byte_string,
    #[regex(r"!")]
    exclamation,
    #[regex(r"#")]
    pound,
    #[regex(r"\$")]
    dollar,
    #[regex(r"%")]
    percent,
    #[regex(r"&")]
    ampersand,
    #[regex(r"\(")]
    open_parenthesis,
    #[regex(r"\)")]
    close_parenthesis,
    #[regex(r"\*")]
    star,
    #[regex(r"\+")]
    plus,
    #[regex(r",")]
    comma,
    #[regex(r"-")]
    minus,
    #[regex(r"\.")]
    dot,
    #[regex(r"/")]
    slash,
    #[regex(r":")]
    colon,
    #[regex(r";")]
    semicolon,
    #[regex(r"<")]
    less,
    #[regex(r"=")]
    equal,
    #[regex(r">")]
    greater,
    #[regex(r"\?")]
    question,
    #[regex(r"@")]
    at,
    #[regex(r"\[")]
    open_bracket,
    #[regex(r"\]")]
    close_bracket,
    #[regex(r"\^")]
    circumflex,
    #[regex(r"\{")]
    open_brace,
    #[regex(r"\|")]
    bar,
    #[regex(r"\}")]
    close_brace,
    #[regex(r"~")]
    tilde,
    #[regex(r"/\*", parse_block_comment)]
    block_comment,
    #[regex(r#"r[#"]"#, parse_raw_string)]
    raw_string,
    #[regex(r#"br[#"]"#, parse_raw_byte_string)]
    raw_byte_string,
    #[error]
    Error,
}

pub struct Lexer;

impl super::Lexer for Lexer {
    fn lex(s: &str) -> Result<super::Token, ()> {
        let lexer = TokenKind::lexer(s).spanned();
        let (kind, span) = { lexer }.next().ok_or(())?;
        assert_eq!(span.start, 0);
        super::Class::ALL
            .get(kind as usize)
            .ok_or(())
            .map(|&class| super::Token {
                class,
                length: span.end,
            })
    }
}

fn parse_block_comment(lex: &mut logos::Lexer<TokenKind>) -> Result<(), &'static str> {
    let mut depth: usize = 1;
    while depth > 0 {
        let interesting_at = lex
            .remainder()
            .find(&['/', '*'][..])
            .ok_or("exhausted source in block comment")?;
        lex.bump(interesting_at);
        if lex.remainder().starts_with("/*") {
            lex.bump(2);
            depth += 1;
        } else if lex.remainder().starts_with("*/") {
            lex.bump(2);
            depth -= 1;
        } else {
            lex.bump(1);
        }
    }
    Ok(())
}

fn parse_raw_string(lex: &mut logos::Lexer<TokenKind>) -> Result<(), &'static str> {
    let hash_count: usize = match lex.slice() {
        r#"r""# => 0,
        r#"r#"# => {
            let mut hashes = 1;
            loop {
                match lex.remainder().chars().next() {
                    Some('#') => {
                        lex.bump(1);
                        hashes += 1;
                    }
                    Some('"') => {
                        lex.bump(1);
                        break;
                    }
                    Some(_) => Err("invalid char in raw string opening fence")?,
                    None => Err("exhausted source in raw string")?,
                }
            }
            hashes
        }
        _ => Err("unreachable")?,
    };

    loop {
        match lex.remainder().chars().next() {
            Some('"') => {
                lex.bump(1);
                let mut hashes_seen: usize = 0;
                loop {
                    if hashes_seen == hash_count {
                        return Ok(());
                    }
                    match lex.remainder().chars().next() {
                        Some('#') => {
                            lex.bump(1);
                            hashes_seen += 1;
                        }
                        _ => break,
                    }
                }
            }
            Some('\r') => {
                lex.bump(1);
                match lex.remainder().chars().next() {
                    Some('\n') => lex.bump(1),
                    _ => Err("bare CR not allowed in raw string")?,
                }
            }
            Some(c) => lex.bump(c.len_utf8()),
            None => Err("exhausted source in raw string")?,
        }
    }
}

fn parse_raw_byte_string(lex: &mut logos::Lexer<TokenKind>) -> Result<(), &'static str> {
    let hash_count: usize = match lex.slice() {
        r#"br""# => 0,
        r#"br#"# => {
            let mut hashes = 1;
            loop {
                match lex.remainder().chars().next() {
                    Some('#') => {
                        lex.bump(1);
                        hashes += 1;
                    }
                    Some('"') => {
                        lex.bump(1);
                        break;
                    }
                    Some(_) => Err("invalid char in raw string opening fence")?,
                    None => Err("exhausted source in raw string")?,
                }
            }
            hashes
        }
        _ => Err("unreachable")?,
    };

    loop {
        match lex.remainder().chars().next() {
            Some('"') => {
                lex.bump(1);
                let mut hashes_seen: usize = 0;
                loop {
                    if hashes_seen == hash_count {
                        return Ok(());
                    }
                    match lex.remainder().chars().next() {
                        Some('#') => {
                            lex.bump(1);
                            hashes_seen += 1;
                        }
                        _ => break,
                    }
                }
            }
            Some('\r') => {
                lex.bump(1);
                match lex.remainder().chars().next() {
                    Some('\n') => lex.bump(1),
                    _ => Err("bare CR not allowed in raw string")?,
                }
            }
            Some(c) => lex.bump(c.len_utf8()),
            None => Err("exhausted source in raw string")?,
        }
    }
}
