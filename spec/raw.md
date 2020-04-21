# Raw lexer

The raw lexer is a transformation from UTF-8 strings to "raw" lexical classes
along with the length of the token with said lexical class.
The lexer and lexical classes are called "raw" because they are not final;
in order to finish lexing Rust source, the tokens must be "cooked" into proper tokens.

The most obvious difference is that the raw lexer does not properly handle floating point
literals. Instead, the cooking step is responsible for gluing together raw tokens to make them.

Each raw lexical class is defined by a regular expression that matches that lexical class.
In the case that two classes both match the prefix of a given string,
ties are explicitly broken by always prefering one class over the other.

There are three lexical classes – `block_comment`, `raw_string`, and `raw_byte_string` –
which are not regular, and thus cannot be specified with just a regular expression.
For these lexical classes, they still have a regular prefix which is used to determine the
lexical class of a string prefix between the regular classes and the nonregular classes.
If the regular prefix of a nonregular class is chosen,
then the nonregular grammar is used to recognize the entire token.

The regular expression used in this specification is as defined by the Rust [regex crate],
which is confined to a fully regular subset of modern extended regex.
All regexes are anchored, meaning they will only match a prefix of the input string.

  [regex crate]: https://docs.rs/regex/1.3.6/regex/

## Regular tokens

For convenience/clarity alone, we define the following named subpatterns.
For actual capturing, they can and should be inlined into the token patterns.

```regex
(?x)(?(DEFINE)
    (?P<quote_escape>   \\['"]                              )
    (?P<named_escape>   \\[nrt\\0]                          )
    (?P<ascii_escape>   \\x[0-7][0-9a-fA-F]                 )
    (?P<byte_escape>    \\x[0-9a-fA-F]{2}                   )
    (?P<unicode_escape> \\u\{(?:10|[0-9])[0-9a-fA-F]{0,4}\} )
    (?P<indent_escape>  \\\n\p{Pattern_White_Space}*        )

    (?P<nonraw_escape>  (?&quote_escape)|(?&named_escape)|(?&ascii_escape)|(?&unicode_escape) )
    (?P<raw_escape>     (?&quote_escape)|(?&named_escape)|(?&byte_escape)                     )
)
```

These subpatterns are referred to using the syntax `(?&name)`.

<dl>
  <dt><code>line_comment</code></dt>
  <dd><code>//[^\n]*</code></dd>

  <dt><code>whitespace</code></dt>
  <dd><code>\p{Pattern_White_Space}+</code></dd>

  <dt><code>identifier</code></dt>
  <dd><code>[_\p{XID_Start}]\p{XID_Continue}*</code></dd>

  <dt><code>raw_identifier</code></dt>
  <dd><code>r#[_\p{XID_Start}]\p{XID_Continue}*</code></dd>

  <dt><code>identifier_fragment</code></dt>
  <dd><code>[\p{XID_Continue}--_0-9\p{XID_Start}]\p{XID_Continue}*</code></dd>

  <dt><code>lifetime</code></dt>
  <dd><code>'[_\p{XID_Start}]\p{XID_Continue}*</code></dd>

  <dt><code>binary_integer</code></dt>
  <dd><code>0b[_0-9]*</code></dd>

  <dt><code>binary_float</code></dt>
  <dd><code>0b[_0-9]*[eE][+-]?[_0-9]*</code></dd>

  <dt><code>octal_integer</code></dt>
  <dd><code>0o[_0-9]*</code></dd>

  <dt><code>octal_float</code></dt>
  <dd><code>0o[_0-9]*[eE][+-]?[_0-9]*</code></dd>

  <dt><code>hexadecimal_integer</code></dt>
  <dd><code>0x[_0-9a-fA-F]*</code></dd>

  <dt><code>decimal_integer</code></dt>
  <dd><code>[0-9][_0-9]*</code></dd>

  <dt><code>decimal_float</code></dt>
  <dd><code>[0-9][_0-9]*[eE][+-]?[_0-9]*</code></dd>

  <dt><code>character</code></dt>
  <dd><code>'(?:[^\t\n\r\\']|(?&amp;nonraw_escape))'</code></dd>

  <dt><code>byte</code></dt>
  <dd><code>'(?:[[:ascii:]--\t\n\r\\']|(?&amp;raw_escape))'</code></dd>

  <dt><code>string</code></dt>
  <dd><code>"(?:[^\r\\"]|\r\n|(?&amp;indent_escape)|(?&amp;nonraw_escape))*"</code></dd>

  <dt><code>byte_string</code></dt>
  <dd><code>b"(?:[[:ascii:]--\r\\"]|\r\n|(?&amp;indent_escape)|(?&amp;raw_escape))*"</code></dd>

  <dt><code>exclamation</code></dt>
  <dd><code>!</code></dd>

  <dt><code>pound</code></dt>
  <dd><code>#</code></dd>

  <dt><code>dollar</code></dt>
  <dd><code>\$</code></dd>

  <dt><code>percent</code></dt>
  <dd><code>%</code></dd>

  <dt><code>ampersand</code></dt>
  <dd><code>&</code></dd>

  <dt><code>open_parenthesis</code></dt>
  <dd><code>\(</code></dd>

  <dt><code>close_parenthesis</code></dt>
  <dd><code>\)</code></dd>

  <dt><code>star</code></dt>
  <dd><code>\*</code></dd>

  <dt><code>plus</code></dt>
  <dd><code>\+</code></dd>

  <dt><code>comma</code></dt>
  <dd><code>,</code></dd>

  <dt><code>minus</code></dt>
  <dd><code>-</code></dd>

  <dt><code>dot</code></dt>
  <dd><code>\.</code></dd>

  <dt><code>slash</code></dt>
  <dd><code>/</code></dd>

  <dt><code>colon</code></dt>
  <dd><code>:</code></dd>

  <dt><code>semicolon</code></dt>
  <dd><code>;</code></dd>

  <dt><code>less</code></dt>
  <dd><code>&lt;</code></dd>

  <dt><code>equal</code></dt>
  <dd><code>=</code></dd>

  <dt><code>greater</code></dt>
  <dd><code>&gt;</code></dd>

  <dt><code>question</code></dt>
  <dd><code>\?</code></dd>

  <dt><code>at</code></dt>
  <dd><code>@</code></dd>

  <dt><code>open_bracket</code></dt>
  <dd><code>\[</code></dd>

  <dt><code>close_bracket</code></dt>
  <dd><code>\]</code></dd>

  <dt><code>circumflex</code></dt>
  <dd><code>\^</code></dd>

  <dt><code>open_brace</code></dt>
  <dd><code>\{</code></dd>

  <dt><code>bar</code></dt>
  <dd><code>\|</code></dd>

  <dt><code>close_brace</code></dt>
  <dd><code>\}</code></dd>

  <dt><code>tilde</code></dt>
  <dd><code>~</code></dd>
</dl>

### Ties

<dl>
  <dt><code>line_comment</code> always ties with <code>slash</code>.</dt>
  <dd>Prefer <code>line_comment</code>, which is always the longer match.</dd>

  <dt><code>binary_integer</code> always ties with <code>decimal_integer</code>.</dt>
  <dd>Prefer <code>binary_integer</code>, which is always the longer match.</dd>

  <dt><code>binary_float</code> always ties with
      <code>binary_integer</code> and <code>decimal_integer</code>.</dt>
  <dd>Prefer <code>binary_float</code>, which is always the longest match.</dd>

  <dt><code>octal_integer</code> always ties with <code>decimal_integer</code>.</dt>
  <dd>Prefer <code>octal_integer</code>, which is always the longer match.</dd>

  <dt><code>octal_float</code> always ties with
      <code>octal_integer</code> and <code>decimal_integer</code>.</dt>
  <dd>Prefer <code>octal_float</code>, which is always the longest match.</dd>

  <dt><code>hexadecimal_integer</code> always ties with <code>decimal_integer</code>.</dt>
  <dd>Prefer <code>hexadecimal_integer</code>, which is always the longer match.</dd>

  <dt><code>decimal_float</code> always ties with <code>decimal_integer</code>.</dt>
  <dd>Prefer <code>decimal_float</code>, which is always the longer match.</dd>

  <dt><code>character</code> can tie with <code>lifetime</code>.</dt>
  <dd>Prefer <code>character</code>, which is always the longer match.</dd>

  <dt><code>byte_string</code> always ties with <code>identifier</code>.</dt>
  <dd>Prefer <code>byte_string</code>, which is always the longer match.</dd>
</dl>

### Note: Reserved Words

Certain identifiers (e.g. `break`, `if`, `fn`, `unsafe`, and others)
are _reserved words_, or as they more commonly referred to, _keywords_.
Lexically they are still identifiers, but the language syntax gives them special meaning,
so the language grammar does not allow using them as typical program identifiers.

## Nonregular tokens' regular prefix

<dl>
  <dt><code>block_comment</code></dt>
  <dd><code>/\*</code><dd>

  <dt><code>raw_string</code></dt>
  <dd><code>r[#"]</code><dd>

  <dt><code>raw_byte_string</code></dt>
  <dd><code>br[#"]</code><dd>
</dl>

### Ties

<dl>
  <dt><code>block_comment</code> always ties with <code>slash</code>.</dt>
  <dd>Prefer <code>block_comment</code>, which is always the longer match.</dd>

  <dt><code>raw_string</code> always ties with <code>identifier</code>.</dt>
  <dd>Prefer <code>raw_string</code>, which is always the longer match.</dd>

  <dt><code>raw_byte_string</code> always ties with <code>identifier</code>.</dt>
  <dd>Prefer <code>raw_byte_string</code>, which is always the longer match.</dd>
</dl>

## Nonregular tokens

The `block_comment`, `raw_string`, and `raw_byte_string` tokens are not regular,
so cannot be fully specified by a regular expression.
Instead, the regular expression recognizes a prefix of the token,
which is used to decide when to match a nonregular token instead of a regular token.

After the regular prefix is recognized, a further recognizer recognizes the full token.
Like the regular expressions, these grammars should be considered anchored, matching a prefix.
These recognizers are specified in [PCRE], [EBNF], and Rust code returning the prefix length.

If a full recognizer fails after the regular prefix is recognized, this is an error.

  [PCRE]: <https://www.pcre.org/>
  [EBNF]: <https://en.wikipedia.org/wiki/Extended_Backus–Naur_form>

### `block_comment`

`block_comment` is a context-free grammar.

Informally: matched pairs of opening `/*` and closing `*/`.

#### PCRE

```regex
(?xs)
(?(DEFINE)(?P<inner> (?:(?! /\* | \*/ ) . )*))
    /\*
    (?P>inner)
    (?: (?R) (?P>inner) )*
    \*/
```

#### EBNF

```ebnf
block comment = open
              , { ({"/"} | {"*"}), char
                | ({"/"} | {"*"}), block comment
                }
              , close
              ;

open  = "/*" ;
close = "*/" ;
char  = ? any character ? - "/" - "*" ;
```

#### Rust

<!-- Note: This is meant to be horribly explicit Rust rather than idiomatic.
     It should be possible to understand with little knowledge of Rust. -->

```rust
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
```

### `raw_string`

`raw_string` is a context-sensitive grammar.

Informally: opened by `r#*"`, closed by first instance of `"#*` with the same number of `#`.

Note: an implementation is allowed to put a limit on the number of `#` allowed in a raw string
(thus making the grammar regular), but at least 2<sup>16</sup> hashes must be supported.
`rustc` uses a limit of 2<sup>16</sup> at the time of writing this specification.
The number of states required to create a DFA matching a raw string with up to _n_ hashes
can be approximated by the function [_0.5n<sup>2</sup> + 2.5n_][state-fn].
For scale, that's on the order of 2<sup>15</sup> states to support 2<sup>8</sup> hashes,
and 2<sup>31</sup> states to support 2<sup>16</sup> hashes.

<!-- 16 =>           168
     25 =>           375
    256 =>        33,408
   2^16 => 2,147,647,488 (2^31 + 2^17 + 2^15)
-->

  [state-fn]: <https://www.wolframalpha.com/input/?i=2n%2BSum%5Bx%2C%7Bx%2C1%2Cn%7D%5D>

#### PCRE

```regex
(?xs)
r(?P<hashes>\#*)"
(?:[^\r]|\r\n)*?
"(?P=hashes)
```

#### EBNF

Note that because `raw_string` is context-sensative, this EBNF has the additional restriction
that only the shortest possible match is produced in ambiguous cases.

```ebnf
raw string = "r", inner ;

inner = "#" , inner , "#"
      | "\"", {char}, "\""
      ;
char  = ? any character ? - "\r"
      | "\r\n"
      ;
```

#### Rust

<!-- Note: This is meant to be horribly explicit Rust rather than idiomatic.
     It should be possible to understand with little knowledge of Rust. -->

```rust
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
                _ => panic!("bare CR not allowed in raw string")
            },
            Some(c) => len += c.len_utf8(),
            None => panic!("exhausted source in raw string"),
        }
    }
}
```

### `raw_byte_string`

`raw_byte_string` is a context-sensitive grammar.

Informally: opened by `br#*"`, closed by first instance of `"#*` with the same number of `#`.
Only ASCII is allowed inside the raw byte string.

Note: an implementation is allowed to put a limit on the number of `#` allowed in a raw string.
See the note on `raw_string` for more context on this limit.

#### PCRE

```regex
(?xs)
br(?P<hashes>\#*)"
(?:[[:ascii:]--\r]|\r\n)*?
"(?P=hashes)
```

#### EBNF

Note that because `raw_byte_string` is context-sensative, this EBNF has the additional restriction
that only the shortest possible match is produced in ambiguous cases.

```ebnf
raw byte string = "br", inner ;

inner = "#" , inner , "#"
      | "\"", {char}, "\""
      ;
char  = ? any ASCII character ? - "\r"
      | "\r\n"
      ;
```

#### Rust

<!-- Note: This is meant to be horribly explicit Rust rather than idiomatic.
     It should be possible to understand with little knowledge of Rust. -->

```rust
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
                _ => panic!("bare CR not allowed in raw byte string")
            },
            Some(c) if c.is_ascii() => len += 1,
            Some(_) => panic!("raw byte string must be ASCII"),
            None => panic!("exhausted source in raw byte string"),
        }
    }
}
```
