# A proposed lexical specification for rust-lang/rust

- [Read the spec](./spec/index.md)
- [See the reference implementations](./src)
- [See the test suite](./tests)

### Todo:

- Reference impl for lexer cooker
- Tests for cooked lexer
- Cooked lexer impl based on `rustc_lexer` (and test against reference impl)
- Cooked lexer impl based on `proc_macro2`'s standalone mode (and test against reference impl)
- Guarantee test suite coverage of all (reachable) arms of the naive reference impl

### Annoyances:

- `identifier_fragment` irks me. But the current setup of matching float exponent syntax in the raw lexer
  (`123e456`) rather than trying to do it in the lexer cooking stage by introspecing identifiers is much cleaner.
  Thus, I think it might be a necessary evil to accept the fun edge case `0.0b·XID_Continue·after·binary·int·raw·class`.

### Nice to haves:

- Generate spec text and reference impl from structured metadata
- More structured test battery, probably homebrew rather than `insta`
- RFC this specification

### Stretch goals?

- Port `rustc_lexer` to use a level similar to raw tokens?
- Autogenerate (part of) `rustc_lexer`?
- Rustc `libsyntax` works with decomposed raw tokens instead of composed tokens?
  (which atm are even more composed than `proc_macro`, `libsyntax` still does token splitting rather than joining) 
- Use raw tokens in grammar spec instead of cooked tokens (`proc_macro`)?
- `meta::tokens` uses raw tokens instead of `proc_macro`'s cooked tokens?
