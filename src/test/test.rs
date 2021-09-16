// use std::fs;
use crate::lexer::lexer::{lex};
use crate::lexer::lexer::Token;
use crate::lexer::lexer::Token::{*};

// let SURF_FILES_PATH = String::from("../../surf-files");

// fn load_test_files() {
//   return fs::read_dir(SURF_FILES_PATH).unwrap();
// }

fn get_tokens<S: Into<String>>(input_string: S) -> Vec<Token> {
  return lex(&input_string.into()).collect();
}

#[test]
pub fn test_lex() {
  assert_eq!(
    get_tokens("const x = 5;"), 
    vec![Const, Identifier("x".to_string()), Equals, Number(5), SemiColon]
  );
}