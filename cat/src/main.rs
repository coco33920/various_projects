use std::env;
use std::env::Args;
use std::error::Error;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let mut args: Args = env::args();
    if args.len() < 2 {
        println!("Usage: cat <file>");
        exit(1);
    }
    let file = args.nth(1);
    match file {
        Some(name) => {
            let s = cat(name);
            if s.is_err() {
                println!("Error while reading file");
            }
        }
        None => println!("Error while reading file")
    };
}

fn cat(file: String) -> Result<(),Box<dyn Error>> {
    let content = read_to_string(file)?;
    println!("{content}");
    Ok(())
}
