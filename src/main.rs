#[derive(Clone, Debug)]
enum OpType {
    Plus,
    Minus,
    Div,
    Times,
}

#[derive(Clone, Debug)]
enum Token {
    Integer(i32),
    Op(OpType),
}

fn eval_line(tokens: Vec<Token>) -> i32 {
    let mut my_tokens = tokens.clone();
    my_tokens.reverse();

    let mut result = match my_tokens.pop().unwrap() {
        Token::Integer(num) => num,
        _ => panic!("Num expected"),
    };

    while let Some(Token::Op(op)) = my_tokens.pop() {
        if let Some(Token::Integer(rval)) = my_tokens.pop() {
            result = match op {
                OpType::Plus => result + rval,
                OpType::Minus => result - rval,
                OpType::Times => result * rval,
                OpType::Div => result / rval,
            }
        } else {
            panic!("Num expected");
        }
    };

    return result;
}

fn main() {
    let tokens = vec! [Token::Integer(22), Token::Op(OpType::Minus), Token::Integer(7), Token::Op(OpType::Div), Token::Integer(3), Token::Op(OpType::Times), Token::Integer(4), Token::Op(OpType::Plus), Token::Integer(1)];
    println!("Initial: {:?}", tokens);
    println!("{}", eval_line(tokens));
}
