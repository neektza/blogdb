use std::str;
use std::iter;

#[derive(Clone, Debug)]
pub struct SyntaxError {
    message: String,
    cnt: u32,
}

#[derive(Clone, Debug)]
pub enum Token {
    Assign,
    OpenParen,
    CloseParen,
    Delimiter,
    Operator(char),
    Literal(i32),
    Identifier(String),
}

pub struct Lexer<'a> {
    chars: iter::Peekable<str::Chars<'a>>,
    tokens: Vec<Token>,
    current: Option<char>,
    cnt: u32,
}

impl<'a> Lexer<'a> {
    pub fn tokenize(s: &str) -> Result<Vec<Token>, SyntaxError> {
        let mut lexer = Lexer {
            chars: s.chars().peekable(),
            tokens: Vec::new(),
            current: None,
            cnt: 0
        };
        try!(lexer.run());
        Ok(lexer.tokens)
    }

    fn current(&self) -> Option<char> {
        self.current
    }
    
    fn advance(&mut self) -> Option<char> {
        self.cnt += 1;
        self.current = self.chars.next();
        return self.current;
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
            if let Some(c) = self.current() {
                if c.is_whitespace() {
                    self.advance();
                    continue;
                } else  {
                    match c {
                        ';' => {
                            self.tokens.push(Token::Delimiter);
                            self.advance();
                        },
                        '=' => {
                            self.tokens.push(Token::Assign);
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
                        '+' | '-' | '*' | '/' | '^' => {
                            self.tokens.push(Token::Operator(c));
                            self.advance();
                        },
                        '0'...'9' => {
                            let val = try!(self.consume_number());
                            self.tokens.push(Token::Literal(val));
                        },
                        'a'...'z' | 'A'...'Z' => {
                            let ident = try!(self.consume_identifier());
                            self.tokens.push(Token::Identifier(ident));
                        },
                        _ => {
                            return Err(SyntaxError { message: String::from("Unknown token."), cnt: self.cnt });
                        }
                    }
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    fn consume_identifier(&mut self) -> Result<String, SyntaxError> {
        let mut s = String::new();

        loop {
            match self.current() {
                Some(c) => {
                    match c {
                        'a'...'z' | 'A'...'Z' | '_' => {
                            s.push(c);
                            self.advance();
                        },
                        _ => {
                            break
                        }
                    }
                },
                None => {
                    return Err(SyntaxError { message: String::from("Malformed identifier."), cnt: self.cnt });
                    break
                }
            }
        }

        Ok(s)
    }

    fn consume_number(&mut self) -> Result<i32, SyntaxError> {
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
            Err(_) => Err(SyntaxError { message: String::from("Not a number"), cnt: self.cnt })
        }
    }
}
