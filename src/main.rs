#[macro_use]
extern crate nom;

mod sql;

fn main() {
    let input = "(12345)";
    let res = sql::parser::parens_num(&input.as_bytes());

    println!("{:?}", res);
}
