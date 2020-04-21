use lexical_specification::raw::{self, reference_impl, Lexer as _};

macro_rules! test_all {
    ($ReferenceLexer:path [$($(#[cfg(feature = $feature:literal)])? $Lexer:path),+ $(,)?] $text:tt) => {
        test_all!(@reference $ReferenceLexer $text);
        $(test_all!(@compare $(#[cfg(feature = $feature)])? $ReferenceLexer, $Lexer $text);)+
    };
    (@reference $Lexer:path [$($text:literal),+ $(,)?]) => {
        $(insta::assert_debug_snapshot!(<$Lexer>::lex_all($text));)+
    };
    (@compare $(#[cfg(feature = $feature:literal)])? $Reference:path, $Lexer:path [$($text:literal),+ $(,)?]) => {
        $(#[cfg(feature = $feature)])?
        {
            $(assert_eq!(<$Reference>::lex_all($text), <$Lexer>::lex_all($text),
                "Reference lexer and {} mismatch on input {:?}",
                stringify!($Lexer), $text);)+
        }
    };
}

#[test]
fn examples_in_informal_reference() {
    test_all! {
        reference_impl::Lexer
        [
            raw::reference_impl::Lexer,
            #[cfg(feature = "logos")] raw::logos_impl::Lexer,
        ]
        [
            // <https://doc.rust-lang.org/stable/reference/keywords.html>
            "as", "break", "const", "continue", "crate", "else", "enum",
            "extern", "false", "fn", "for", "if", "impl", "in", "let",
            "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
            "self", "Self", "static", "struct", "super", "trait", "true",
            "type", "unsafe", "use", "where", "while", "async", "await",
            "dyn", "abstract", "become", "box", "do", "final", "macro",
            "override", "priv", "typeof", "unsized", "virtual", "yield",
            "try", "union", "'static", "dyn",
            // <https://doc.rust-lang.org/stable/reference/comments.html>
            "\
//! A doc comment that applies to the implicit anonymous module of this crate

pub mod outer_module {

//!  - Inner line doc
//!! - Still an inner line doc (but with a bang at the beginning)

/*!  - Inner block doc */
/*!! - Still an inner block doc (but with a bang at the beginning) */

//   - Only a comment
///  - Outer line doc (exactly 3 slashes)
//// - Only a comment

/*   - Only a comment */
/**  - Outer block doc (exactly) 2 asterisks */
/*** - Only a comment */

pub mod inner_module {}

pub mod nested_comments {
    /* In Rust /* we can /* nest comments */ */ */

    // All three types of block comments can contain or be nested inside
    // any other type:

    /*   /* */  /** */  /*! */  */
    /*!  /* */  /** */  /*! */  */
    /**  /* */  /** */  /*! */  */
    pub mod dummy_item {}
}

pub mod degenerate_cases {
    // empty inner line doc
    //!

    // empty inner block doc
    /*!*/

    // empty line comment
    //

    // empty outer line doc
    ///

    // empty block comment
    /**/

    pub mod dummy_item {}

    // empty 2-asterisk block isn't a doc block, it is a block comment
    /***/

}

/* The next one isn't allowed because outer doc comments
    require an item that will receive the doc */

/// Where is my item?
}",
            // <https://doc.rust-lang.org/stable/reference/whitespace.html>
            "\u{0009}", "\u{000A}", "\u{000B}", "\u{000C}", "\u{000D}",
            "\u{0020}", "\u{0085}", "\u{200E}", "\u{200F}", "\u{2028}",
            "\u{2029}",
            // <https://doc.rust-lang.org/stable/reference/tokens.html>
            "98_222", "0xff", "0o77", "0b1111_0000", "123.0E+77", "1_234.0E+18f64", r###""foo""###, r###"r"foo""###, r###""\"foo\"""###, r###"r#""foo""#"###, r###""foo #\"# bar""###, r###"r##"foo #"# bar"##"###, r###""\x52""###, r###""R""###, r###"r"R""###, r###""\\x52""###, r###"r"\x52""###, r###"b"foo""###, r###"br"foo""###, r###"b"\"foo\"""###, r###"br#""foo""#"###, r###"b"foo #\"# bar""###, r###"br##"foo #"# bar"##"###, r###"b"\x52""###, r###"b"R""###, r###"br"R""###, r###"b"\\x52""###, r###"br"\x52""###, "123", "123i32", "123u32", "123_u32", "0xff", "0xff_u8", "0o70", "0o70_i16", "0b1111_1111_1001_0000", "0b1111_1111_1001_0000i64", "0b________1", "0usize", "0invalidSuffix", "123AFB43", "0b0102", "0o0581", "128_i8", "256_u8", "0b_", "0b____", "123.0f64", "0.1f64", "0.1f32", "12E+99_f64", "2.",
        ]
    }
}
