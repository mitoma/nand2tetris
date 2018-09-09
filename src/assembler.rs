extern crate nand2tetlis;

use std::collections;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let program_path = &env::args().nth(1);
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

fn assemble(program_path: &str) {
    let f = fs::File::open(program_path).unwrap();
    let reader = BufReader::new(f);

    //let symbol_table = collections::HashMap::new();

    for line in reader.lines() {
        let line = line.map(|l| l.trim().to_owned());
        match line {
            Ok(line) => {
                match &line {
                    v if v.starts_with("//") => {} // コメント
                    v if v.starts_with("@") => println!("A命令"),
                    v if v.starts_with("(") && v.ends_with(")") => println!("Loop"),
                    v if v.is_empty() => {}
                    v => println!("{}", parse_c_command(v)),
                }
            }
            _ => {}
        };
    }
}

fn parse_c_command(command: &str) -> String {
    println!("org_command:{}", command);

    let command = &command;
    let dest = match command {
        v if v.starts_with("M=") => "001",
        v if v.starts_with("D=") => "010",
        v if v.starts_with("MD=") => "011",
        v if v.starts_with("A=") => "100",
        v if v.starts_with("AM=") => "101",
        v if v.starts_with("AD=") => "110",
        v if v.starts_with("AMD=") => "111",
        _ => "000",
    };
    let jump = match command {
        v if v.ends_with(";JGT") => "001",
        v if v.ends_with(";JEQ") => "010",
        v if v.ends_with(";JGE") => "011",
        v if v.ends_with(";JLT") => "100",
        v if v.ends_with(";JNE") => "101",
        v if v.ends_with(";JLE") => "110",
        v if v.ends_with(";JMP") => "111",
        _ => "000",
    };
    let command = match command.find("=") {
        Some(v) => command.split_at(v + 1).1,
        _ => &command,
    };
    let command = match command.find(";") {
        Some(v) => command.split_at(v).0,
        _ => command,
    };
    let comp = match command {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "D-1" => "0001111",
        "A-1" => "0110011",
        "D+A" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" => "0000000",
        "D|A" => "0010101",
        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" => "1110111",
        "M-1" => "1110011",
        "D+M" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "D|M" => "1010101",
        _ => "",
    };
    println!(
        "comp:{}, dest:{}, jump:{}, command:{}",
        comp, dest, jump, command
    );
    format!("111{}{}{}", comp, dest, jump)
}
