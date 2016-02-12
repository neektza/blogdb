#[derive(Clone, Debug)]
enum MultiplicativeOp {
    Mul,
    Div,
}

#[derive(Clone, Debug)]
enum AdditiveOp {
    Add,
    Sub,
}

#[derive(Clone, Debug)]
enum Token {
    AddOp(AdditiveOp),
    MultOp(MultiplicativeOp),
    Identifier(String),
    Integer(i32),
    Error(String),
}

/* ----------------------------- LEXER BEGIN */

fn consume_number(current_char: char, iter: &mut std::str::Chars) -> Option<i32> {
    let mut peekable_iter = iter.peekable();
    let mut num:Vec<char> = Vec::new();

    num.push(current_char);

    while let Some(item) = peekable_iter.next() {
        num.push(item);

        if let Some(next_item) = peekable_iter.peek() {
            if !next_item.is_digit(10) {
                break;
            }
        }
    }

    let num_string = num.iter().cloned().collect::<String>();
    return num_string.parse::<i32>().ok();
}

fn tokenize(line: &str) -> Vec<Token> {
    let mut iter = line.chars();
    let mut tokens = Vec::new();
    let mut token;

    while let Some(c) = iter.next() {
        if c.is_whitespace() { continue };

        if c.is_digit(10) {
            token = match consume_number(c, &mut iter) {
                Some(i32) => Token::Integer(i32),
                None => Token::Error(String::from("error scanning digits")),
            };
        } else {
            token = match c {
                '+' => Token::AddOp(AdditiveOp::Add),
                '-' => Token::AddOp(AdditiveOp::Sub),
                '*' => Token::MultOp(MultiplicativeOp::Mul),
                '/' => Token::MultOp(MultiplicativeOp::Div),
                _   => Token::Error(String::from("unknown token")),
            };
        };
        tokens.push(token);
    }
    return tokens;
}


/* ----------------------------- LEXER END */

#[derive(Clone, Debug)]
struct Factor(i32);

#[derive(Clone, Debug)]
struct Term (Factor, Vec<(MultiplicativeOp, Factor)>);

#[derive(Clone, Debug)]
struct Expr (Term, Vec<(AdditiveOp, Term)>);

/* ----------------------------- PARSER BEGIN */

fn parse_factor(mut iter: &mut std::slice::Iter<Token>) -> Factor {
    if let Some(t) = iter.next() {
        match *t {
            Token::Integer(i) => Factor(i),
            _ => panic!("not a factor"),
        }
    } else {
        panic!("parsing error");
    }
        
}

fn parse_term(mut iter: &mut std::slice::Iter<Token>) -> Term {

    let first = parse_factor(&mut iter);
    let mut rest : Vec<(MultiplicativeOp, Factor)> = Vec::new();

    while let Some(t) = iter.next() {
        if let Token::MultOp(ref o) = *t {
            let tmp = match o {
                &MultiplicativeOp::Mul => (MultiplicativeOp::Mul, parse_factor(&mut iter)),
                &MultiplicativeOp::Div => (MultiplicativeOp::Div, parse_factor(&mut iter)),
            };
            rest.push(tmp);
        } else {
            println!("U ELSE TERM SAM");
            ()
        }
    }

    return Term(first, rest);
}

fn parse_expr(tokens: Vec<Token>) -> Expr {
    let mut iter = tokens.iter();

    let first = parse_term(&mut iter);
    let mut rest : Vec<(AdditiveOp, Term)> = Vec::new();

    while let Some(t) = iter.next() {
        if let Token::AddOp(ref o) = *t {
            let tmp = match o {
                &AdditiveOp::Add => (AdditiveOp::Add, parse_term(&mut iter)),
                &AdditiveOp::Sub => (AdditiveOp::Sub, parse_term(&mut iter)),
            };
            rest.push(tmp);
        } else {
            rest.push();
            ()
        }

    }

    return Expr(first, rest);
}

/* ----------------------------- PARSER END */

fn main() {
    /* EXAMPLE 1 */

    let line1 = "631 * 212";
    println!("{:?}", line1);

    let tokens1 = tokenize(&line1);

    let ast1 = parse_expr(tokens1);
    println!("{:?}", ast1);

    /* EXAMPLE 2 */
    
    let line2 = "631 + 212";
    println!("{:?}", line2);

    let tokens2 = tokenize(&line2);

    let ast2 = parse_expr(tokens2);
    println!("{:?}", ast2);

    /* EXAMPLE 3 */
    
    let line3 = "742 * 631 + 212";
    println!("{:?}", line3);

    let tokens3 = tokenize(&line3);

    let ast3 = parse_expr(tokens3);
    println!("{:?}", ast3);
    
    /* EXAMPLE 4 */
    
    let line4 = "742 + 641 * 212";
    println!("{:?}", line4);

    let tokens4 = tokenize(&line4);

    let ast4 = parse_expr(tokens4);
    println!("{:?}", ast4);
}


