use std::env;
use std::env::Args;
use std::fs::{DirEntry, read_dir};
use std::process::exit;
use crate::LsArg::{COLOR, SORT};

enum LsArg {
    COLOR,
    SORT,
}

fn main() {
    let mut args: Args = env::args();
    if args.len() < 2 {
        println!("Usage: ls <dir> [COLOR TIME SORT]");
        exit(1);
    }
    let dir = args.nth(1);

    let ls_args = parse_args(args);

    match dir {
        Some(name) => ls(name),
        None => println!("Error!")
    };
}

fn parse_args(args: Args) -> Vec<LsArg> {
    let mut ls_args: Vec<LsArg> = Vec::new();
    if args.len() > 2 {
        args.for_each(
            |arg| {
                let argument = parse_arg(&arg);
                match argument {
                    Some(option) => ls_args.push(option),
                    None => ()
                }
            }
        )
    }
    ls_args
}

fn parse_arg(arg: &str) -> Option<LsArg> {
    match arg {
        "-c" => Some(COLOR),
        "--color" => Some(COLOR),
        "-s" => Some(SORT),
        "--sort" => Some(SORT),
        _ => return None
    }
}

fn ls(dir: String) {
    let files = read_dir(dir).unwrap();
    for file in files {
        print_dir_entry(file.unwrap())
    }
}

fn print_dir_entry(entry: DirEntry) {
    let file_name = entry.file_name()
        .into_string()
        .unwrap()
        .replace("\"", "");
    println!("{file_name}");
}

