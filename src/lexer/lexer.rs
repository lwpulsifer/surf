use logos::{Logos};

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

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("fn")]
    Function,

    // -----------
    // Punctuation
    // -----------

    #[token(":")]
    Colon,

    #[token(";")]
    SemiColon,

    #[token(".")]
    Accessor,

    // -----------
    // Operators
    // -----------

    #[token("=")]
    Assigns,

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

    // -----------
    // Comparison Operators
    // -----------

    #[token("==")]
    Equals,

    #[token("!=")]
    NotEquals,

    #[token("<=")]
    LessOrEqual,

    #[token(">=")]
    GreaterOrEqual,

    #[token("<")]
    Less,

    #[token(">")]
    Greater,

    #[token("is")]
    Is,

    #[token("and")]
    And,

    #[token("or")]
    Or,

    #[token("xor")]
    Xor,


    // -------------------
    // User-entered values
    // -------------------
    #[
      regex(
        r"[a-zA-Z0-9_?]*", 
        |lex| lex.slice().parse()
      )
    ]
    Identifier(String),

    #[regex("-?[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),

    #[regex("-?[0-9]+\\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),

    #[regex("[\"][^\"]*[\"]", |lex| lex.slice().to_string())]
    #[regex("['][^']*[']", |lex| lex.slice().to_string())]
    #[regex("[`][^`]*[`]", |lex| lex.slice().to_string())]
    Str(String),

    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub fn lex(filecontents: &String) -> logos::Lexer<Token> {
  return Token::lexer(&filecontents);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Token::{*};

    fn get_tokens<S: Into<String>>(input_string: S) -> Vec<Token> {
      return lex(&input_string.into()).collect();
    }

    #[test]
    fn simple_const_declaration() {
      assert_eq!(
        get_tokens("const x = 5;"),
        vec![Const, Identifier("x".to_string()), Assigns, Integer(5), SemiColon]
      );
    }

    #[test]
    fn simple_let_declaration() {
      assert_eq!(
        get_tokens("let tiger = true;"),
        vec![Let, Identifier("tiger".to_string()), Assigns, True, SemiColon]
      );
    }

    #[test]
    fn simple_let_with_type() {
      assert_eq!(
        get_tokens("let tiger: int = 100;"),
        vec![Let, Identifier("tiger".to_string()), Colon, Identifier("int".to_string()), Assigns, Integer(100), SemiColon]
      );
    }

    #[test]
    fn simple_float_dec() {
      assert_eq!(
        get_tokens("let f = 5.89"),
        vec![Let, Identifier("f".to_string()), Assigns, Float(5.89)]
      )
    }

    #[test]
    fn complex_float_dec() {
      assert_eq!(
        get_tokens("let f = -194.5810230"),
        vec![Let, Identifier("f".to_string()), Assigns, Float(-194.5810230)]
      )
    }

    #[test]
    fn simple_string_dec() {
      assert_eq!(
        get_tokens("let s = 'Hi there!'"),
        vec![Let, Identifier("s".to_string()), Assigns, Str("'Hi there!'".to_string())]
      )
    }

    #[test]
    fn complex_var_name() {
      assert_eq!(
        get_tokens("let _tigerStyle49000 = 8;"),
        vec![Let, Identifier("_tigerStyle49000".to_string()), Assigns, Integer(8), SemiColon]
      )
    }

    #[test]
    fn number_initialized_var_name() {
      assert_eq!(
        get_tokens("const 10Tiger = 4;"),
        vec![Const, Identifier("10Tiger".to_string()), Assigns, Integer(4), SemiColon]
      )
    }

    #[test]
    fn arithmetic_expression() {
      assert_eq!(
        get_tokens("5 + 10 - 54 /_ 6 / 1000 % 3"),
        vec![Integer(5), Plus, Integer(10), Minus, Integer(54), FloorDivide, Integer(6), Divide, Integer(1000), Modulo, Integer(3)]
      )
    }

    #[test]
    fn accessor() {
      assert_eq!(
        get_tokens("let x = obj.x"),
        vec![Let, Identifier("x".to_string()), Assigns, Identifier("obj".to_string()), Accessor, Identifier("x".to_string())]
      )
    }
}