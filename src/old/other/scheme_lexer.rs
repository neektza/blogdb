use std::str;
use std::fmt;
use std::iter;

pub fn tokenize(s: &str) -> Result<Vec<Token>, SyntaxError> {
    Lexer::tokenize(s)
}

#[derive(PartialEq, Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    Op,
    Integer(i64),
}

pub struct SyntaxError {
    message: String,
    line: u32,
    column: u32,
}

struct Lexer<'a> {
    chars: iter::Peekable<str::Chars<'a>>,
    current: Option<char>,
    tokens: Vec<Token>,
    line: u32,
    column: u32,
}

impl<'a> Lexer<'a> {
    fn tokenize(s: &str) -> Result<Vec<Token>, SyntaxError> {
        let mut lexer = Lexer {
            chars: s.chars().peekable(),
            current: None,
            tokens: Vec::new(),
            line: 1,
            column: 0
        };
        try!(lexer.run());
        Ok(lexer.tokens)
    }

    fn current(&self) -> Option<char> {
        self.current
    }

    fn advance(&mut self) {
        if self.current() == Some('\x0a') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.current = self.chars.next();
    }

    fn peek(&mut self) -> Option<char> {
        match self.chars.peek() {
            Some(c) => Some(*c),
            None => None
        }
    }

    fn run(&mut self) -> Result<(), SyntaxError> {
        self.advance();
        loop {
            match self.current() {
                Some(c) => {
                    match c {
                        _ if c.is_whitespace() => {
                            self.advance();
                        },
                        '(' => {
                            self.tokens.push(Token::OpenParen);
                            self.advance();
                        },
                        ')' => {
                            self.tokens.push(Token::CloseParen);
                            self.advance();
                        },
                        },
                        '+' | '-' | '*' | '/' => {
                            match self.peek() {
                                Some('0'...'9') => {
                                    // skip past the +/- symbol and parse the number
                                    self.advance();
                                    let val = try!(self.parse_number());
                                    self.tokens.push(Token::Integer(if c == '-' { -1 * val } else { val }));
                                    try!(self.parse_delimiter());
                                },
                                _ => {
                                    syntax_error!(self, "Expected number after Op", c);
                                }
                            }
                        },
                        '0'...'9' => {
                            // don't advance -- let parse_number advance as needed
                            let val = try!(self.parse_number());
                            self.tokens.push(Token::Integer(val));
                            try!(self.parse_delimiter());
                        },
                        '[' | ']' | '{' | '}' | '|' | '\\' => {
                            syntax_error!(self, "Unexpected character: {}", c);
                        },
                        _ => {
                            let val = try!(self.parse_identifier());
                            self.tokens.push(Token::Identifier(val));
                            try!(self.parse_delimiter());
                        }
                    }
                },
                None => break
            }
        };
        Ok(())
    }

    fn parse_number(&mut self) -> Result<i64, SyntaxError> {
        let mut s = String::new();
        loop {
            match self.current() {
                Some(c) => {
                    match c {
                        '0'...'9' => {
                            s.push(c);
                            self.advance();
                        },
                        _ => break
                    }
                },
                None => break
            }
        }
        match s.parse() {
            Ok(value) => Ok(value),
            Err(_) => { syntax_error!(self, "Not a number: {}", self.current().unwrap()); },
        }
    }
}

macro_rules! syntax_error {
    ($lexer:ident, $($arg:tt)*) => (
        return Err(SyntaxError { message: format!($($arg)*), line: $lexer.line, column: $lexer.column })
    )
}
