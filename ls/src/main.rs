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
        Some(name) => ls(name, &ls_args),
        None => println!("Error!")
    };
}

fn parse_args(args: Args) -> Vec<LsArg> {
    let mut ls_args: Vec<LsArg> = Vec::new();
    args.for_each(
        |arg| {
            let argument = parse_arg(&arg);
            match argument {
                Some(option) => ls_args.push(option),
                None => ()
            }
        }
    );
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

fn ls(dir: String, options: &Vec<LsArg>) {
    let mut files: Vec<String> = Vec::new();
    let mut dirs: Vec<String> = Vec::new();
    let _ = read_dir(dir)
        .unwrap()
        .for_each(|file| {
            let (a, b) = print_dir_entry(file.unwrap(), options);
            match a {
                Some(str) => files.push(str),
                None => ()
            };
            match b {
                Some(str) => dirs.push(str),
                None => ()
            };
        });
    for option in options {
        match option {
            COLOR => (),
            SORT => {
                files.sort();
                dirs.sort();
            }
        }
    }
    for dir in dirs {
        println!("{dir}");
    }
    for file in files {
        println!("{file}");
    }
}

fn option_string(file_name: String, is_dir: bool) -> (Option<String>, Option<String>) {
    if is_dir { return (None, Some(file_name)); } else { return (Some(file_name), None); }
}

fn print_dir_entry(entry: DirEntry, options: &Vec<LsArg>) -> (Option<String>, Option<String>) {
    let file_name = entry.file_name()
        .into_string()
        .unwrap()
        .replace("\"", "");
    let is_dir = entry.file_type()
        .unwrap()
        .is_dir();

    let binding = {
        let color = if is_dir { ansi_term::Colour::Blue.normal() } else { ansi_term::Colour::White.normal() };
        color.paint(&file_name)
    };

    if options.len() == 0 {
        return option_string(file_name, is_dir);
    }
    let mut a: (Option<String>, Option<String>) = option_string(file_name.clone(), is_dir);
    options.iter().for_each(|x| {
        match x {
            COLOR => a = option_string(binding.to_string(), is_dir),
            SORT => ()
        }
    });
    a
}

