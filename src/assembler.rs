extern crate nand2tetlis;

use std::collections;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let program_path = env::args().nth(1);
    match program_path {
        Some(path) => assemble(path),
        None => {
            println!(
                "{}",
                "hack バイナリのパスを指定してください"
            );
            std::process::exit(0)
        }
    }
}

fn assemble(program_path: String) {
    let f = fs::File::open(program_path).unwrap();
    let reader = BufReader::new(f);

    let symbol_table = collections::HashMap::new();
    for line in reader.lines() {
        match line.unwrap().trim() {
            v if v.starts_with("//") => {}
            v if v.starts_with("@") => println!("A命令"),
            v if v.starts_with("(") && v.ends_with(")") => println!("Loop"),
            v => println!("{}", v),
            _ => {}
        }
    }
}
