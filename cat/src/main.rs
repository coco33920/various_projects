use std::env;
use std::env::Args;
use std::fs::{read_to_string};
use std::process::exit;

fn main() {
    let mut args: Args = env::args();
    if args.len() < 2 {
        println!("Usage: cat <file>");
        exit(1);
    }
    let file = args.nth(1);
    match file {
        Some(name) => cat(name),
        None => println!("Error!")
    };
}

fn cat(file: String) {
    let file_read = read_to_string(file).unwrap();
    print!("{file_read}")
}