use std::io;
use std::io::{BufRead, Write};
use std::process::exit;

use ansi_term::Color;

use crate::lexer::lex;

mod lexer;
mod token;

fn main() {
    let message = Color::Blue.paint("Welcome to calc v0.1.0 by Charlotte Thomas \ntype help for getting help for the commands\n");
    println!("{}", message.to_string());
    loop {
        print!("{}", Color::Cyan.paint("> "));
        let _ = io::stdout().flush();
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        handle.read_line(&mut buffer).expect("TODO: panic message");
        match buffer.as_str().trim() {
            "info" => {
                let message = Color::Purple.paint(" Calc v0.1.0 \n Author: Charlotte Thomas \n Written in Rust \n Repo: https://github.com/coco33920/various_projects\n");
                println!("{}", message)
            }
            "exit" => break,
            "help" => {
                let message = Color::Purple.paint(
                    " Calc v0.1.0 Help \n > info : show infos \n > exit : exit the program \n > help : print this help \n"
                );
                println!("{}",message)
            }
            str => {
                let a = lex(str.to_string());
                println!("Lexing of line: {str}");
                println!("{:?}", a);
                println!()
            }
        }
    }
    exit(0);
}
