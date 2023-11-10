use std::env;
use std::env::Args;
use std::fs::{DirEntry, read_dir};
use std::process::exit;

fn main() {
    let mut args: Args = env::args();
    if args.len() < 2 {
        println!("Usage: ls <dir>");
        exit(1);
    }
    let dir = args.nth(1);
    match dir {
        Some(name) => ls(name),
        None => println!("Error!")
    };
}
fn ls(dir: String) {
    let files = read_dir(dir).unwrap();
    for file in files {
        print_dir_entry(file.unwrap())
    }
}

fn print_dir_entry(entry: DirEntry){
    let file_name = entry.file_name()
        .into_string()
        .unwrap()
        .replace("\"","");
    println!("{file_name}");
}

