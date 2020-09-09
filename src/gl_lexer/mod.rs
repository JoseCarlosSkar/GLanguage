use crate::gl_token::Token;
use crate::gl_token_position::TokenPosition;

pub struct Lexer {
	filename: String,
	codetext: String,
	chars: Vec<char>,
	linestext: Vec<String>,
	position: TokenPosition,
	current_char: String,
	current_linetext: String,
	tokens: Vec<Token>,
}

impl Lexer {
	pub fn new(filename: String, codetext: String) -> Lexer {
		let mut chars: Vec<char> = codetext.chars().collect::<Vec<char>>();
		let mut linestext: Vec<String> = codetext.lines().map(|line| line.to_string()).collect::<Vec<String>>();
		let position: TokenPosition = TokenPosition::new(0, 0, 0);
		let current_char: String = chars.remove(0).to_string();
		let current_linetext: String = if linestext.len() > 0 { linestext.remove(0) } else { String::from(&codetext) };
		let tokens: Vec<Token> = Vec::new();
		Lexer { filename, codetext, chars, linestext, position, current_char, current_linetext, tokens }
	}
}
