use crate::gl_token::Token;
use crate::gl_token_position::TokenPosition;
use crate::gl_tokens::{is_token, Tokens, DIGITS, LETTERS, LETTERS_DIGITS, PUNCTUATIONS, SPACES};

pub struct Lexer {
	filename: String,
	chars: Vec<char>,
	linestext: Vec<String>,
	pub position: TokenPosition,
	current_char: String,
	current_linetext: String,
	pub tokens: Vec<Token>,
}

impl Lexer {
	pub fn new(filename: String, codetext: String, lineno: u32) -> Lexer {
		let mut chars: Vec<char> = codetext.chars().collect::<Vec<char>>();
		let mut linestext: Vec<String> = codetext.lines().map(|line| line.to_string()).collect::<Vec<String>>();
		let position: TokenPosition = TokenPosition::new(0, lineno, 0);
		let current_char: String = if chars.len() > 0 { chars.remove(0).to_string() } else { String::new() };
		let current_linetext: String = if linestext.len() > 0 { linestext.remove(0) } else { String::from(&codetext) };
		let tokens: Vec<Token> = Vec::new();
		Lexer { filename, chars, linestext, position, current_char, current_linetext, tokens }
	}

	fn advance_position(&mut self) {
		self.position.index += 1;
		self.position.column += 1;
	}

	fn advance_char(&mut self) {
		if self.chars.len() > 0 {
			self.current_char = self.chars.remove(0).to_string();
		} else {
			self.current_char = String::new();
		}
	}

	fn advance_linetext(&mut self) {
		self.position.lineno += 1;
		self.position.index = 0;
		self.position.column = 0;
		if self.linestext.len() > 0 {
			self.current_linetext = self.linestext.remove(0);
		}
	}

	fn advance(&mut self) {
		self.advance_position();
		self.advance_char();
	}

	fn build_new_token(&mut self, typer: Tokens, pos_start: TokenPosition) -> Token {
		let pos_end: TokenPosition = self.position.copy();
		let token: Token = Token::new(typer, String::from(&self.filename), String::from(&self.current_linetext), pos_start, pos_end);
		self.tokens.push(token.copy());
		token
	}

	fn illegal_char(&mut self) {
		let pos_start: TokenPosition = self.position.copy();
		let token: Token = self.build_new_token(Tokens::EOF, pos_start);
		self.advance_linetext();
		token.illegal_char();
	}

	fn invalid_syntax(&mut self) {
		let pos_start: TokenPosition = self.position.copy();
		let token: Token = self.build_new_token(Tokens::EOF, pos_start);
		self.advance_linetext();
		token.invalid_syntax(String::new());
	}

	pub fn copy_tokens(&self) -> Vec<Token> {
		let mut tokens: Vec<Token> = Vec::new();
		for t in self.tokens.iter() {
			tokens.push(t.copy());
		}
		tokens
	}

	fn make_next_token(&mut self) -> bool {
		let pos_start: TokenPosition = self.position.copy();

		if self.current_char.is_empty() {
			if self.tokens.len() > 0 {
				self.build_new_token(Tokens::EOF, self.tokens[self.tokens.len() - 1].position_end.copy());
			} else {
				self.build_new_token(Tokens::EOF, pos_start);
			}
			return false;
		} else if SPACES.contains(&self.current_char.as_str()) {
			if self.current_char == "\n" {
				self.advance_linetext();
				self.advance_char();
			} else {
				self.advance();
			}
		} else if DIGITS.contains(&self.current_char.as_str()) {
			let mut number_string = String::new();
			while self.current_char.is_empty() == false && DIGITS.contains(&self.current_char.as_str()) {
				number_string.push_str(&self.current_char.as_str());
				self.advance();
			}
			if self.current_char.is_empty() == false
				&& SPACES.contains(&self.current_char.as_str()) == false
				&& PUNCTUATIONS.contains(&self.current_char.as_str()) == false
			{
				if is_token(&self.current_char.as_str()) {
					self.invalid_syntax();
				} else {
					self.illegal_char();
				}
				return true;
			}
			self.build_new_token(Tokens::INTEGER(number_string), pos_start);
		} else if LETTERS.contains(&self.current_char.as_str()) {
			let mut identifier_string = String::new();
			while self.current_char.is_empty() == false && LETTERS_DIGITS.contains(&self.current_char.as_str()) {
				identifier_string.push_str(&self.current_char.as_str());
				self.advance();
			}
			if self.current_char.is_empty() == false
				&& SPACES.contains(&self.current_char.as_str()) == false
				&& PUNCTUATIONS.contains(&self.current_char.as_str()) == false
			{
				if is_token(&self.current_char.as_str()) {
					self.invalid_syntax();
				} else {
					self.illegal_char();
				}
				return true;
			}
			self.build_new_token(Tokens::IDENTIFIER(identifier_string), pos_start);
		} else if PUNCTUATIONS.contains(&self.current_char.as_str()) {
			if self.current_char == ";" {
				self.advance();
				self.build_new_token(Tokens::SEMICOLON, pos_start);
			} else if self.current_char == "(" {
				self.advance();
				self.build_new_token(Tokens::LPAREN, pos_start);
			} else if self.current_char == ")" {
				self.advance();
				self.build_new_token(Tokens::RPAREN, pos_start);
			} else if self.current_char == "," {
				self.advance();
				self.build_new_token(Tokens::COMMA, pos_start);
			} else if self.current_char == "{" {
				self.advance();
				self.build_new_token(Tokens::LBRACE, pos_start);
			} else if self.current_char == "}" {
				self.advance();
				self.build_new_token(Tokens::RBRACE, pos_start);
			}
		} else if self.current_char == "\"" {
			let mut literal_string: String = String::new();
			let mut escape_character: bool = false;
			self.advance();
			while self.current_char.is_empty() == false && (self.current_char != "\"" || escape_character == true) {
				if escape_character == true {
					if self.current_char == "n" {
						literal_string += "\n";
					} else {
						literal_string += self.current_char.as_str();
						escape_character = false;
					}
				} else {
					if self.current_char == "\\" {
						escape_character = true;
					} else {
						literal_string += self.current_char.as_str();
					}
				}
				self.advance();
			}

			if self.current_char != "\"" {
				self.invalid_syntax();
				return true;
			}
			self.advance();
			if self.current_char.is_empty() == false
				&& SPACES.contains(&self.current_char.as_str()) == false
				&& PUNCTUATIONS.contains(&self.current_char.as_str()) == false
			{
				if is_token(&self.current_char.as_str()) {
					self.invalid_syntax();
				} else {
					self.illegal_char();
				}
				return true;
			}
			self.build_new_token(Tokens::STRING(literal_string), pos_start.copy());
		} else {
			self.illegal_char();
			return true;
		}

		return self.make_next_token();
	}

	pub fn run(&mut self) -> bool {
		if self.make_next_token() == true {
			return true;
		}
		return false;
	}
}
