use crate::lexer::lex;

mod lexer;
mod token;

fn main() {
    println!("{:?}",lex("314.05".to_string()))
}
