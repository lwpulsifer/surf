use std::env;
use std::fs;
use std::process;

mod lexer;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  if !filename.ends_with(".surf") {
    println!("Expecting a file with extension 'surf', but found {}", filename);
    process::exit(1);
  }
  let _filecontents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file.");

  // Lex

  let lex = lexer::lexer::lex(&_filecontents);
  for token in lex {
    println!("Token: {:?}", token);
  }

  // Parse

  // Do more cool stuff

  println!("Ran {}", filename);
}
