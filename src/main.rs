use std::io;
use std::io::prelude::*;

use std::string::String;

enum OpType {
    Plus,
    Minus,
    Div,
    Times,
}

enum Token {
    Integer(i32),
    Op(OpType),
}

fn tokenize(line: String) -> Vec<Token> {
    let tokens = vec! [Token::Integer(7), Token::Op(OpType::Plus), Token::Integer(3)];
    return tokens;
}

fn evalLine(tokens: Vec<Token>) -> i32 {

    for token in tokens {
        match token {
            Token::Integer(i) => println!("{}", i),
            Token::Op(OpType::Plus) => println!("PLUS"),
            Token::Op(OpType::Minus) => println!("MINUS"),
            Token::Op(OpType::Times) => println!("TIMES"),
            Token::Op(OpType::Div) => println!("DIV"),
        }
    }

    return 12;
}

fn interpret(line: String) -> i32 {
    let tokens = tokenize(line);
    return evalLine(tokens);
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // println!("{}", line.unwrap());
        println!("{}", interpret(line.unwrap()));
    }
}
