// From: https://www.reddit.com/r/rust/comments/2ow729/writing_a_lexer/
use std::iter::Peekable;

fn main() {
    let input = "abcdABCDabcd,".chars();

    let tokenizer = Tokenizer::new(input);

    for token in tokenizer {
        println!("{:?}", token);
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    GroupL,
    GroupR,
    Separator,
    Implies,
    Variable(String),
    Atom(String),
}

pub struct Tokenizer<I: Iterator<Item=char>> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item=char>> Tokenizer<I> {
    pub fn new(iter: I) -> Tokenizer<I> {
        Tokenizer {
            iter: iter.peekable(),
        }
    }
}

impl<I: Iterator<Item=char>> Iterator for Tokenizer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match *self.iter.peek().unwrap_or(&'⊥') {
            '('       => {Some(Token::GroupL)},
            ')'       => {Some(Token::GroupR)},
            ','       => {Some(Token::Separator)},
            '←'       => {Some(Token::Implies)},
            'A'...'Z' => Some(Token::Atom(self.iter.by_ref()
                            .take_while(|&c| c.is_uppercase()).collect())),
            'a'...'z' => Some(Token::Atom(self.iter.by_ref()
                            .take_while(|&c| c.is_lowercase()).collect())),
            '⊥'       => None,
             _        => self.next(),
        }
    }
}
