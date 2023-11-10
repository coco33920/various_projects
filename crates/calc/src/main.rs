use crate::lexer::lex;

mod lexer;
mod token;

fn main() {
    println!("{:?}",lex("100+".to_string()))
}
