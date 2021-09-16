use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // Tokens can be literal strings, of any length.
    
    // -----------
    // Keywords
    // -----------

    #[token("let")]
    Let,

    #[token("const")]
    Const,

    #[token("true")]
    True,

    #[token("false")]
    False,

    // -----------
    // Punctuation
    // -----------

    #[token(":")]
    Colon,

    #[token(";")]
    SemiColon,

    // -----------
    // Operators
    // -----------

    #[token("=")]
    Equals,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Times,

    #[token("/")]
    Divide,

    #[token("/_")]
    FloorDivide,

    #[token("%")]
    Modulo,

    // -------------------
    // User-entered values
    // -------------------
    #[
      regex(
        r"[a-zA-Z_][a-zA-Z0-9_?]*", 
        |lex| lex.slice().parse()
      )
    ]
    Identifier(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Number(i64),

    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex("[0-9]+[a-z-A-Z0-9_?]")]
    Error,
}

pub fn lex(filecontents: &String) -> logos::Lexer<Token> {
  return Token::lexer(&filecontents);
}