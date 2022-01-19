use logos::Lexer;
use crate::lexer::lexer::Token;
use Token::*;

#[derive(Debug)]
pub enum ASTNode {
	// LetDec { name: String, value: Box<ASTNode> },
	// ConstDec { name: String, value: Box<ASTNode> },

	IntExp { value: i64 },
	StringExp { value: String },
	BoolExp { value: bool },
}

enum ParsedValue {
	Done,
	Success { node: ASTNode },
}

pub fn parse(lex: &mut Lexer<Token>) -> Vec<ASTNode> {
	let mut root = Vec::new(); 
	loop {
		let current_parse_value: ParsedValue = parse_exp(lex);
		match current_parse_value {
			ParsedValue::Done => break,
			ParsedValue::Success { node } => root.push(node)
		}
	}
	return root;
}

fn parse_exp(lex: &mut Lexer<Token>) -> ParsedValue {
	let node;
	match lex.next() {
		Some(token) => node = match token {
			Integer(x) => ASTNode::IntExp { value: x },
			Str(s) => ASTNode::StringExp { value: s },
			_ => panic!("Unexpected token")
		},
		None => return ParsedValue::Done,
	}
	return ParsedValue::Success { node };
	
}

// fn parse_let(lex: Lexer<Token>) -> ParsedValue {
// 	let var_name = match lex.next() {
// 		Some(token) => match token {
// 			Identifier(name) => name,
// 		_ => panic!("Let value must be an identifier")
// 		},
// 		None => panic!("Unclosed let!")
// 	};
// 	expect(lex, Assigns);
// 	let subParse = parse_exp(lex);
// 	let subExp;
// 	match subParse {
// 		ParsedValue::Done => return ParsedValue::Done,
// 		ParsedValue::Failure { errorMsg } => return ParsedValue::Failure { errorMsg },
// 		ParsedValue::Success { node } => subExp = node
// 	}
// 	let LetDec = ASTNode::LetDec { name: var_name, value: Box::new(subExp) };
// 	expect(lex, SemiColon);
// 	ParsedValue::Success { node: LetDec }
// }

// fn parse_const(lex: Lexer<Token>) -> ParsedValue {
// 	return ParsedValue::Done;
// }

// fn expect(lex: Lexer<Token>, token: Token) -> Token {
// 	let next_token = lex.next();
// 	if (next_token.unwrap() != token) {
// 		println!("Expected {:?}, got {:?}", token, next_token);
// 		panic!();
// 	}
// 	return token;
// }