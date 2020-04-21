use {
    super::*,
    once_cell::sync::Lazy,
    regex::{Regex, RegexSet},
    std::panic::catch_unwind,
};

static REGEX_SET: Lazy<RegexSet> = Lazy::new(|| {
    RegexSet::new(vec![
        r"\A//[^\n]*",                                               // line_comment
        r"\A\p{Pattern_White_Space}+",                               // whitespace
        r"\A[_\p{XID_Start}]\p{XID_Continue}*",                      // identifier
        r"\Ar#[_\p{XID_Start}]\p{XID_Continue}*",                    // raw_identifier
        r"\A[\p{XID_Continue}--_0-9\p{XID_Start}]\p{XID_Continue}*", // identifier_fragment
        r"\A'[_\p{XID_Start}]\p{XID_Continue}*",                     // lifetime
        r"\A0b[_0-9]*",                                              // binary_integer
        r"\A0b[_0-9]*[eE][+-]?[_0-9]*",                              // binary_float
        r"\A0o[_0-9]*",                                              // octal_integer
        r"\A0o[_0-9]*[eE][+-]?[_0-9]*",                              // octal_float
        r"\A0x[_0-9a-fA-F]*",                                        // hexadecimal_integer
        r"\A0x[_0-9a-fA-F]*[eE][+-]?[_0-9a-fA-F]*",                  // hexadecimal_float
        r"\A[0-9][_0-9]*",                                           // decimal_integer
        r"\A[0-9][_0-9]*[eE][+-]?[_0-9]*",                           // decimal_float
        concat!(
            r#"\A'(?:[^\t\n\r\\']"#,
            r#"|\\['"]"#,                              // quote_escape
            r#"|\\[nrt\\0]"#,                          // named_escape
            r#"|\\x[0-7][0-9a-fA-F]"#,                 // ascii_escape
            r#"|\\u\{(?:10|[0-9])[0-9a-fA-F]{0,4}\}"#, // unicode_escape
            r#")'"#
        ), // character
        concat!(
            r#"\A'(?:[[:ascii:]--\t\n\r\\']"#,
            r#"|\\['"]"#,            // quote_escape
            r#"|\\[nrt\\0]"#,        // named_escape
            r#"|\\x[0-9a-fA-F]{2}"#, // byte_escape
            r#")'"#
        ), // byte
        concat!(
            r#"\A"(?:[^\t\n\r\\"]"#,
            r#"|\r\n"#,
            r#"|\\\n\p{Pattern_White_Space}*"#, // indent_escape
            r#"|\\['"]"#,                       // quote_escape
            r#"|\\[nrt\\0]"#,                   // named_escape
            r#"|\\x[0-7][0-9a-fA-F]"#,          // ascii_escape
            r#"|\\u\{(?:10|[0-9])[0-9a-fA-F]{0,4}\}"#, // unicode_escape
            r#")*""#
        ), // string
        concat!(
            r#"\Ab"(?:[[:ascii:]--\r\\"]"#,
            r#"|\r\n"#,
            r#"|\\\n\p{Pattern_White_Space}*"#, // indent_escape
            r#"|\\['"]"#,                       // quote_escape
            r#"|\\[nrt\\0]"#,                   // named_escape
            r#"|\\x[0-9a-fA-F]{2}"#,            // byte_escape
            r#")*""#
        ), // byte_string
        r"\A!",                                                      // exclamation
        r"\A#",                                                      // pound
        r"\A\$",                                                     // dollar
        r"\A%",                                                      // percent
        r"\A&",                                                      // ampersand
        r"\A\(",                                                     // open_parenthesis
        r"\A\)",                                                     // close_parenthesis
        r"\A\*",                                                     // star
        r"\A\+",                                                     // plus
        r"\A,",                                                      // comma
        r"\A-",                                                      // minus
        r"\A\.",                                                     // dot
        r"\A/",                                                      // slash
        r"\A:",                                                      // colon
        r"\A;",                                                      // semicolon
        r"\A<",                                                      // less
        r"\A=",                                                      // equal
        r"\A>",                                                      // greater
        r"\A\?",                                                     // question
        r"\A@",                                                      // at
        r"\A\[",                                                     // open_bracket
        r"\A\]",                                                     // close_bracket
        r"\A\^",                                                     // circumflex
        r"\A\{",                                                     // open_brace
        r"\A\|",                                                     // bar
        r"\A\}",                                                     // close_brace
        r"\A~",                                                      // tilde
        r"\A/\*",                                                    // block_comment
        r#"\Ar[#"]"#,                                                // raw_string
        r#"\Abr[#"]"#,                                               // raw_byte_string
    ])
    .unwrap()
});

static ALL_REGEX: Lazy<Vec<Regex>> = Lazy::new(|| {
    let mut vec = vec![];
    for i in 0..REGEX_SET.patterns().len() {
        vec.push(Regex::new(&REGEX_SET.patterns()[i]).unwrap());
    }
    vec
});

pub struct Lexer;

impl super::Lexer for Lexer {
    fn lex(s: &str) -> Result<Token, ()> {
        let matches: Vec<Class> = REGEX_SET
            .matches(s)
            .iter()
            .map(|ix| Class::ALL[ix])
            .collect();
        let class = match &*matches {
            [] => Err(())?,
            &[any] => any,
            [line_comment, slash] => line_comment,
            [binary_integer, decimal_integer] => binary_integer,
            [binary_integer, binary_float, decimal_integer] => binary_float,
            [octal_integer, decimal_integer] => octal_integer,
            [octal_integer, octal_float, decimal_integer] => octal_float,
            [hexadecimal_integer, decimal_integer] => hexadecimal_integer,
            [hexadecimal_integer, hexadecimal_float, decimal_integer] => hexadecimal_float,
            [decimal_integer, decimal_float] => decimal_float,
            [lifetime, character] => character,
            [identifier, byte_string] => byte_string,
            [identifier, identifier_fragment] => identifier,
            [slash, block_comment] => block_comment,
            [identifier, raw_string] => raw_string,
            [identifier, raw_byte_string] => raw_byte_string,
            other => unimplemented!("raw lexer matched set {:?}", other),
        };
        Ok(match class {
            block_comment => Token {
                class,
                length: catch_unwind(|| parse_block_comment(s)).map_err(drop)?,
            },
            raw_string => Token {
                class,
                length: catch_unwind(|| parse_raw_string(s)).map_err(drop)?,
            },
            raw_byte_string => Token {
                class,
                length: catch_unwind(|| parse_raw_byte_string(s)).map_err(drop)?,
            },
            _ => {
                let mat = ALL_REGEX[class as usize].find(s).unwrap();
                assert_eq!(mat.start(), 0);
                Token {
                    class,
                    length: mat.end(),
                }
            }
        })
    }
}

// Reminder: these functions are copied from the specification.
// Do not optimize or make idiomatic; their purpose is to be obvious.

pub fn parse_block_comment(s: &str) -> usize {
    let mut chars = s.chars().peekable();
    assert_eq!(chars.next(), Some('/'));
    assert_eq!(chars.next(), Some('*'));

    let mut depth: usize = 1;
    let mut len: usize = 2;

    while depth > 0 {
        match chars.next() {
            Some('/') if matches!(chars.peek(), Some('*')) => {
                chars.next();
                depth += 1;
                len += 2;
            }
            Some('*') if matches!(chars.peek(), Some('/')) => {
                chars.next();
                depth -= 1;
                len += 2;
            }
            Some(c) => len += c.len_utf8(),
            None => panic!("exhausted source in block comment"),
        }
    }

    return len;
}

fn parse_raw_string(s: &str) -> usize {
    let mut chars = s.chars().peekable();
    assert_eq!(chars.next(), Some('r'));

    let mut hashes: usize = 0;
    let mut len: usize = 1;

    loop {
        match chars.next() {
            Some('#') => {
                len += 1;
                hashes += 1;
            }
            Some('"') => {
                len += 1;
                break;
            }
            Some(c) => panic!("invalid char {:?} in raw string opening fence", c),
            None => panic!("exhausted source in raw string"),
        }
    }

    loop {
        match chars.next() {
            Some('"') => {
                len += 1;
                let mut hashes_seen: usize = 0;
                loop {
                    if hashes_seen == hashes {
                        return len;
                    }
                    match chars.peek() {
                        Some('#') => {
                            chars.next();
                            len += 1;
                            hashes_seen += 1;
                        }
                        _ => break,
                    }
                }
            }
            Some('\r') => match chars.next() {
                Some('\n') => len += 2,
                _ => panic!("bare CR not allowed in raw string"),
            },
            Some(c) => len += c.len_utf8(),
            None => panic!("exhausted source in raw string"),
        }
    }
}

fn parse_raw_byte_string(s: &str) -> usize {
    let mut chars = s.chars().peekable();
    assert_eq!(chars.next(), Some('b'));
    assert_eq!(chars.next(), Some('r'));

    let mut hashes: usize = 0;
    let mut len: usize = 2;

    loop {
        match chars.next() {
            Some('#') => {
                len += 1;
                hashes += 1;
            }
            Some('"') => {
                len += 1;
                break;
            }
            Some(c) => panic!("invalid char {:?} in raw byte string opening fence", c),
            None => panic!("exhausted source in raw byte string"),
        }
    }

    loop {
        match chars.next() {
            Some('"') => {
                len += 1;
                let mut hashes_seen: usize = 0;
                loop {
                    if hashes_seen == hashes {
                        return len;
                    }
                    match chars.peek() {
                        Some('#') => {
                            chars.next();
                            len += 1;
                            hashes_seen += 1;
                        }
                        _ => break,
                    }
                }
            }
            Some('\r') => match chars.next() {
                Some('\n') => len += 2,
                _ => panic!("bare CR not allowed in raw byte string"),
            },
            Some(c) if c.is_ascii() => len += 1,
            Some(_) => panic!("raw byte string must be ASCII"),
            None => panic!("exhausted source in raw byte string"),
        }
    }
}
