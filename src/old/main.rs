mod lexer;
mod parser;

fn main() {
    /* EXAMPLE 1 */

    let line1 = "iks = 7; ipsilon = 3; (631 + iks) * (482 + ipsilon^3)";
    println!("{:?}", line1);

    let tokens = lexer::Lexer::tokenize(&line1);
    println!("{:?}", tokens);
    
    let ast = parser::Parser::parse(&tokens.unwrap());
    println!("{:?}", ast);

    // let ast1 = parse(tokens1);
    // println!("{:?}\n", ast1);

    // /* EXAMPLE 2 */
    
    // let line2 = "631 + 212";
    // println!("{:?}", line2);

    // let tokens2 = tokenize(&line2);
    // // println!("{:?}", tokens2);

    // let ast2 = parse(tokens2);
    // println!("{:?}\n", ast2);

    // /* EXAMPLE 3 */
    
    // let line3 = "742 * 631 + 212";
    // println!("{:?}", line3);

    // let tokens3 = tokenize(&line3);
    // // println!("{:?}", tokens3);

    // let ast3 = parse(tokens3);
    // println!("{:?}\n", ast3);
    
    // /* EXAMPLE 4 */
    
    // let line4 = "742 + 641 * 212";
    // println!("{:?}", line4);

    // let tokens4 = tokenize(&line4);
    // // println!("{:?}", tokens4);

    // let ast4 = parse(tokens4);
    // println!("{:?}\n", ast4);
    
    // /* EXAMPLE 4 */
    
    // let line5 = "752 + 651 * 212 + 935";
    // println!("{:?}", line5);

    // let tokens5 = tokenize(&line5);
    // // println!("{:?}", tokens5);

    // let ast5 = parse(tokens5);
    // println!("{:?}\n", ast5);
}


