# Rust lexical specification

The Rust lexical specification is split into two parts:

1. The "[raw lexer]," which is a formally specified parser of UTF-8 strings to lexical classes.
3. The "[lexer cooker]," which is a formally specified transformation from the raw lexer's output to a cooked token stream.

Throughout the spec, tokens are not entirely opaque. Higher levels may make decisions based on the
textual content of lower level's output tokens. For example, the lexical specification makes no
provision for keywords. Instead, it is the grammar's repsonsibility to prevent the use of reserved
identifiers (keywords) where they should not be allowed.

The lexical structure produced by this specification is roughly equivalent to that provided by
[`proc_macro::TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html),
but with explicit whitespace and comments, and without matching brackets up.

Ed note: Except for the case of reserved words, it's probably a good idea to minimize
the use of nonopaque tokens in the specification. We don't want to have to "rediscover"
lexical information that an earlier stage threw away.

  [raw lexer]: ./raw.md
  [lexer cooker]: ./cooked.md
