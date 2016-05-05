use lexer::*;
use std::fmt;
use std::iter;
use std::slice;

macro_rules! parse_error {
    ($($arg:tt)*) => (
        return Err(ParseError { message: format!($($arg)*)})
    )
}

#[derive(Clone, Debug)]
pub enum MultOp { Mul, Div }

#[derive(Clone, Debug)]
pub enum AddtOp { Add, Sub }

#[derive(Clone, Debug)]
pub struct Factor(i32);

#[derive(Clone, Debug)]
pub struct Term (Factor, Vec<(MultOp, Factor)>);

#[derive(Clone, Debug)]
pub enum Expr {
    ExprNumeric(Term, Vec<(AddtOp, Term)>),
    ExprAssignment { identifier: String, value: Factor }
}

#[derive(Clone)]
pub struct ParseError {
    message: String,
}

pub struct Parser<'a> {
    tokens: iter::Peekable<slice::Iter<'a, Token>>,
    ast: Vec<Expr>,
}

impl <'a> Parser<'a> {
    pub fn parse(tokens: &Vec<Token>) -> Result<Vec<Expr>, ParseError> {
        let mut parser = Parser {
            tokens: tokens.iter().peekable(),
            ast: Vec::new()
        };
        return Ok(vec![])
    }

    fn parse_nodes(&mut self) -> Result<Vec<Expr>, ParseError> {
        let ast = Vec::new();

        loop {
            match self.tokens().next() {
                Some(t) => {
                    match t {
                        Token::Identifier => {
                        },
                        Token::Literal => {
                        }
                    }
                },
                None => { return Ok(ast); }
            }
        }
    }
    
    fn parse_assignment(&mut self) -> Result<Option<Expr>, ParseError> {
        return Ok(Some(Expr::ExprAssignment { identifier: String::from("a"), value: Factor(75) }));
    }

    fn parse_expr(&mut self) -> Result<Option<Expr>, ParseError> {
        let factor = Factor(323);
        let term = Term(factor, vec![]);
        let expr = Expr::ExprNumeric(term, vec![]);

        let some_expr = Some(expr);
        return Ok(some_expr);

    }
    
    fn parse_term(&mut self) -> Result<Term, ParseError> {
        return Ok(Term(Factor(11), vec![]));
    }


    fn parse_factor(&mut self) -> Result<Factor, ParseError> {
        return Ok(Factor(7));
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}
