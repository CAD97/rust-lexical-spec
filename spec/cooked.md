# Lexer cooker

The lexer cooker takes as input the raw tokens as produced by the [raw lexer].
Cooked tokens are produced by repeatedly matching the front of the raw token stream
against the following cases, and matching case is used to pop raw tokens off of the
stream and yield cooked tokens.

In some cases, more than one lexer cooking match may be applicable.
In all such cases, one such match will match more raw tokens than the other.
The match that consumes more raw tokens is preferred.

For clarity, raw tokens are indicated using `lower_snake_case`,
and cooked tokens are indicated using `TitleCase`.
Logical groupings of cooked tokens are named using `Namespaced::TitleCase`.
This is just for indicating logical grouping and has no impact on semantics.

The special cooked token `ERROR` represents a required lexer error.
For convenience, a nonnormative description of what error is being caught is provided.

  [raw lexer]: ./raw.md

### Unprocessed

<dl>
  <dt><code>whitespace</code></dt>
  <dd><code>Trivia::Whitesapce</code></dd>

  <dt><code>identifier</code></dt>
  <dd><code>Identifier::Plain</code></dd>

  <dt><code>raw_identifier</code></dt>
  <dd><code>Identifier::Raw</code></dd>

  <dt><code>lifetime</code></dt>
  <dd><code>Lifetime</code></dd>

  <dt><code>exclamation</code> (<code>!</code>)</dt>
  <dd><code>Punctuation::Exclamation</code></dd>

  <dt><code>pound</code> (<code>#</code>)</dt>
  <dd><code>Punctuation::Pound</code></dd>

  <dt><code>dollar</code> (<code>$</code>)</dt>
  <dd><code>Punctuation::Dollar</code></dd>

  <dt><code>percent</code> (<code>%</code>)</dt>
  <dd><code>Punctuation::Percent</code></dd>

  <dt><code>ampersand</code> (<code>&</code>)</dt>
  <dd><code>Punctuation::Ampersand</code></dd>

  <dt><code>open_parenthesis</code> (<code>(</code>)</dt>
  <dd><code>Punctuation::Parenthesis::Open</code></dd>

  <dt><code>close_parenthesis</code> (<code>)</code>)</dt>
  <dd><code>Punctuation::Parenthesis::Close</code></dd>

  <dt><code>star</code> (<code>*</code>)</dt>
  <dd><code>Punctuation::Star</code></dd>

  <dt><code>plus</code> (<code>+</code>)</dt>
  <dd><code>Punctuation::Plus</code></dd>

  <dt><code>comma</code> (<code>,</code>)</dt>
  <dd><code>Punctuation::Comma</code></dd>

  <dt><code>minus</code> (<code>-</code>)</dt>
  <dd><code>Punctuation::Minus</code></dd>

  <dt><code>dot</code> (<code>.</code>)</dt>
  <dd><code>Punctuation::Dot</code></dd>

  <dt><code>slash</code> (<code>/</code>)</dt>
  <dd><code>Punctuation::Slash</code></dd>

  <dt><code>colon</code> (<code>:</code>)</dt>
  <dd><code>Punctuation::Colon</code></dd>

  <dt><code>semicolon</code> (<code>;</code>)</dt>
  <dd><code>Punctuation::Semicolon</code></dd>

  <dt><code>less</code> (<code>&lt;</code>)</dt>
  <dd><code>Punctuation::Less</code></dd>

  <dt><code>equal</code> (<code>=</code></dt>
  <dd><code>Punctuation::Equal</code></dd>

  <dt><code>greater</code> (<code>&gt;</code>)</dt>
  <dd><code>Punctuation::Greater</code></dd>

  <dt><code>question</code> (<code>?</code>)</dt>
  <dd><code>Punctuation::Question</code></dd>

  <dt><code>at</code> (<code>@</code>)</dt>
  <dd><code>Punctuation::At</code></dd>

  <dt><code>open_bracket</code> (<code>[</code>)</dt>
  <dd><code>Punctuation::Bracket::Open</code></dd>

  <dt><code>close_bracket</code> (<code>]</code>)</dt>
  <dd><code>Punctuation::Bracket::Close</code></dd>

  <dt><code>circumflex</code> (<code>^</code>)</dt>
  <dd><code>Punctuation::CircumFlex</code></dd>

  <dt><code>close_brace</code> (<code>{</code>)</dt>
  <dd><code>Punctuation::Brace::Open</code></dd>

  <dt><code>bar</code> (<code>|</code>)</dt>
  <dd><code>Punctuation::|</code></dd>

  <dt><code>close_brace</code> (<code>}</code>)</dt>
  <dd><code>Punctuation::Brace::Close</code></dd>

  <dt><code>tilde</code> (<code>~</code>)</dt>
  <dd><code>Punctuation::~</code></dd>
</dl>

### Miscelaneous Processed

<dl>
  <dt><code>line_comment</code> where prefix is
      <code>///</code></dt>
  <dd><code>Documentation::Outer::Line</code></dd>

  <dt><code>line_comment</code> where prefix is
      <code>//!</code></dt>
  <dd><code>Documentation::Inner::Line</code></dd>

  <dt><code>line_comment</code> where prefix is not
      <code>///</code> or <code>//!</code></dt>
  <dd><code>Trivia::Comment::Line</code></dd>

  <dt><code>block_comment</code> where prefix is
      <code>/**</code></dt>
  <dd><code>Documentation::Outer::Block</code></dd>

  <dt><code>block_comment</code> where prefix is
      <code>/*!</code></dt>
  <dd><code>Documentation::Inner::Block</code></dd>

  <dt><code>block_comment</code> where prefix is not
      <code>/**</code> or <code>/*!</code></dt>
  <dd><code>Trivia::Comment::Block</code></dd>

  <dt><code>lifetime lifetime</code></dt>
  <dd><code>ERROR</code> (character literal may only contain one codepoint)</dd>

  <dt><code>character</code></dt>
  <dd><code>Literal::Character::Unsuffixed</code></dd>

  <dt><code>character identifier</code></dt>
  <dd><code>Literal::Character::Suffixed</code></dd>

  <dt><code>byte</code></dt>
  <dd><code>Literal::Byte::Unsuffixed</code></dd>

  <dt><code>byte identifier</code></dt>
  <dd><code>Literal::Byte::Suffixed</code></dd>

  <dt><code>string</code></dt>
  <dd><code>Literal::String::Unsuffixed</code></dd>

  <dt><code>string identifier</code></dt>
  <dd><code>Literal::String::Suffixed</code></dd>

  <dt><code>byte_string</code></dt>
  <dd><code>Literal::ByteString::Unsuffixed</code></dd>

  <dt><code>byte_string identifier</code></dt>
  <dd><code>Literal::ByteString::Suffixed</code></dd>

  <dt><code>raw_string</code></dt>
  <dd><code>Literal::RawString::Unsuffixed</code></dd>

  <dt><code>raw_string identifier</code></dt>
  <dd><code>Literal::RawString::Suffixed</code></dd>

  <dt><code>raw_byte_string</code></dt>
  <dd><code>Literal::RawByteString::Unsuffixed</code></dd>

  <dt><code>raw_byte_string_identifier</code></dt>
  <dd><code>Literal::RawByteString::Suffixed</code></dd>
</dl>

### Floating Point

Ed note: floating point is hard

#### Not Floating Point

<dl>
  <dt><code>binary_integer dot</code></dt>
  <dd><code>ERROR</code> (Binary float literals not supported)</dd>

  <dt><code>binary_integer dot dot</code></dt>
  <dd><code>Literal::Integer::Binary Punctuation::Dot Punctuation::Dot</code></dd>

  <dt><code>binary_integer dot identifier</code></dt>
  <dd><code>Literal::Integer::Binary Punctuation::Dot Identifier::Plain</code></dd>

  <dt><code>binary_integer dot raw_identifier</code></dt>
  <dd><code>Literal::Integer::Binary Punctuation::Dot Identifier::Raw</code></dd>

  <dt><code>octal_integer dot</code></dt>
  <dd><code>ERROR</code> (Octal float literals not supported)</dd>

  <dt><code>octal_integer dot dot</code></dt>
  <dd><code>Literal::Integer::Octal Punctuation::Dot Punctuation::Dot</code></dd>

  <dt><code>octal_integer dot identifier</code></dt>
  <dd><code>Literal::Integer::Octal Punctuation::Dot Identifier::Plain</code></dd>

  <dt><code>octal_integer dot raw_identifier</code></dt>
  <dd><code>Literal::Integer::Octal Punctuation::Dot Identifier::Raw</code></dd>

  <dt><code>hexadecimal_integer dot</code></dt>
  <dd><code>ERROR</code> (Hexadecimal float literals not supported)</dd>

  <dt><code>hexadecimal_integer dot dot</code></dt>
  <dd><code>Literal::Integer::Hexadecimal Punctuation::Dot Punctuation::Dot</code></dd>

  <dt><code>hexadecimal_integer dot identifier</code></dt>
  <dd><code>Literal::Integer::Hexadecimal Punctuation::Dot Identifier::Plain</code></dd>

  <dt><code>hexadecimal_integer dot raw_identifier</code></dt>
  <dd><code>Literal::Integer::Hexadecimal Punctuation::Dot Identifier::Raw</code></dd>

  <dt><code>binary_integer identifier</code> where the identifier has a prefix of
      <code>e</code> or <code>E</code></dt>
  <dd><code>ERROR</code> (Binary float literals not supported)</dd>

  <dt><code>octal_integer identifier</code> where the identifier has a prefix of
      <code>e</code> or <code>E</code></dt>
  <dd><code>ERROR</code> (Octal float literals not supported)</dd>

  <dt><code>hexadecimal_integer identifier</code> where the identifier has a prefix of
      <code>e</code> or <code>E</code></dt>
  <dd><code>ERROR</code> (Hexadecimal float literals not supported)</dd>

  <dt><code>decimal_integer identifier</code> where the identifier has a prefix of
      <code>e</code> or <code>E</code> and the identifier does not match
      <code>^[eE]_*[0-9]</code></dt>
  <dd><code>ERROR</code> (No digits in float literal exponent)</dd>

  <dt><code>decimal_integer dot decimal_integer identifier</code> where the identifier
      has a prefix of <code>e</code> or <code>E</code> and the identifier does not match
      <code>^[eE]_*[0-9]</code></dt>
  <dd><code>ERROR</code> (No digits in float literal exponent)</dd>
</dl>

#### Actually Floating Point

<dl>
  <dt><code>decimal_integer dot</code></dt>
  <dd><code>Literal::Float::Unsuffixed</code></dd>

  <dt><code>decimal_integer dot decimal_integer</code></dt>
  <dd><code>Literal::Float::Unsuffixed</code></dd>

  <dt><code>decimal_integer identifier</code> where the identifier totally matches
      <code>[eE]_*[0-9][_0-9]*</code></dt>
  <dd><code>Literal::Float::Unsuffixed</code></dd>

  <dt><code>decimal_integer identifier</code> where the identifier matches
      <code>^[eE]_*[0-9]</code> but not <code>^[eE]_*[0-9][_0-9]*$</code></dt>
  <dd><code>Literal::Float::Suffixed</code></dd>

  <dt><code>decimal_integer dot decimal_integer identifier</code> where the identifier
      totally matches <code>[eE]_*[0-9][_0-9]*</code></dt>
  <dd><code>Literal::Float::Unsuffixed</code></dd>

  <dt><code>decimal_integer dot decimal_integer identifier</code> where the identifier
      matches <code>^[eE]_*[0-9]</code> but not <code>^[eE]_*[0-9][_0-9]*$</code></dt>
  <dd><code>Literal::Float::Suffixed</code></dd>
</dl>

### Numeric Literals

#### Not Numeric Literals

<dl>
  <dt><code>binary_integer</code> where the token contains no match for
  <code>[0-9]</code> after the <code>b</code></dt>
  <dd><code>ERROR</code> (No digits in binary integer literal)</dd>

  <dt><code>binary_integer</code> where the token contains a match for
  <code>[2-9]</code> after the <code>b</code></dt>
  <dd><code>ERROR</code> (Invalid digit for binary integer literal)</dd>

  <dt><code>octal_integer</code> where the token contains no match for
  <code>[0-9]</code> after the <code>b</code></dt>
  <dd><code>ERROR</code> (No digits in octal integer literal)</dd>

  <dt><code>octal_integer</code> where the token contains a match for
  <code>[8-9]</code> after the <code>b</code></dt>
  <dd><code>ERROR</code> (Invalid digit for octal integer literal)</dd>

  <dt><code>hexadecimal_integer</code> where the token contains no match for
  <code>[0-9a-fA-f]</code> after the <code>x</code></dt>
  <dd><code>ERROR</code> (No digits in hexadecimal integer literal)</dd>

  <dt><code></code></dt>
  <dd><code></code></dd>
</dl>

#### Actually Numeric Literals

<dl>
  <dt><code>binary_integer</code> where the entire token matches
      <code>0b_*[01][_01]*</code></dt>
  <dd><code>Literal::Integer::Binary::Unsuffixed</code></dd>

  <dt><code>binary_integer identifier</code> where the <code>binary_integer</code> token matches
      <code>^0b_*[01][_01]*$</code> and the <code>identifier</code> does not have a prefix of
      <code>e</code> or <code>E</code></dt>
  <dd><code>Literal::Integer::Binary::Suffixed</code> </dd>

  <dt><code>octal_integer</code> where the entire token matches
      <code>0o_*[0-7][_0-7]*</code></dt>
  <dd><code>Literal::Integer::Octal::Unsuffixed</code></dd>

  <dt><code>octal_integer identifier</code> where the <code>octal_integer</code> token matches
      <code>^0o_*[0-7][_0-7]*$</code> and the <code>identifier</code> does not have a prefix of
      <code>e</code> or <code>E</code></dt>
  <dd><code>Literal::Integer::Octal::Suffixed</code></dd>

  <dt><code>hexadecimal_integer</code> where the entire token matches
      <code>0x_*[0-9a-fA-F][_0-9a-fA-F]*</code></dt>
  <dd><code>Literal::Integer::Hexadecimal::Unsuffixed</code></dd>

  <dt><code>hexadecimal_integer identifier</code> where the <code>hexadecimal_integer</code>
      token matches <code>^0x_*[0-9a-fA-F][_0-9a-fA-F]*$</code> and the <code>identifier</code>
      does not have a prefix of <code>e</code> or <code>E</code></dt>
  <dd><code>Literal::Integer::Hexadecimal::Suffixed</code></dd>

  <dt><code>decimal_integer</code></dt>
  <dd><code>Literal::Integer::Decimal::Unsuffixed</code></dd>

  <dt><code>decimal_integer identifier</code> where the <code>identifier</code>
      does not have a prefix of <code>e</code> or <code>E</code></dt>
  <dd><code>Literal::Integer::Decimal::Suffixed</code></dd>
</dl>
