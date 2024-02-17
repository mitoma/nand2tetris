use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about = "hack assembler", long_about = None)]
struct Args {
    /// hack program path
    program_path: String,
}

enum Command<'a> {
    Comment(&'a str),
    Argument(&'a str),
    Control(&'a str),
    Loop(&'a str),
}

fn main() {
    let args = Args::parse();
    assemble(&args.program_path);
}

fn assemble(program_path: &str) {
    let f = fs::File::open(program_path).unwrap();
    let reader = BufReader::new(f);

    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| match l.find("//") {
            Some(v) => l[0..v].to_string(),
            None => l,
        })
        .map(|l| l.trim().to_owned())
        .collect();

    let commands: Vec<Command> = lines.iter().map(|l| parse_command(l)).collect();

    let mut symbol_table = create_symble_table(&commands);

    let mut current_symbol_address = 16;
    for command in &commands {
        let asm_line = match command {
            Command::Argument(symbol_name) => {
                if symbol_name.parse::<u16>().is_ok() {
                    Some(format!("{:016b}", symbol_name.parse::<u16>().unwrap()))
                } else {
                    if !symbol_table.contains_key(symbol_name) {
                        symbol_table.insert(symbol_name, current_symbol_address);
                        current_symbol_address += 1;
                    }
                    Some(format!(
                        "{:016b}",
                        parse_a_command(symbol_name, &symbol_table)
                    ))
                }
            }
            Command::Control(v) => Some(parse_c_command(v)),
            Command::Loop(_) | Command::Comment(_) => None,
        };
        if let Some(line) = asm_line {
            println!("{line}");
        }
    }
}

fn parse_command(line: &str) -> Command {
    match line {
        v if v.starts_with("//") => Command::Comment(v),
        v if v.starts_with('@') => {
            let symbol_name = &v[1..];
            Command::Argument(symbol_name)
        }
        v if v.starts_with('(') && v.ends_with(')') => {
            let symbol_name = v.trim_matches(&['(', ')'] as &[_]);
            Command::Loop(symbol_name)
        }
        "" => Command::Comment("empty"),
        v => Command::Control(v),
    }
}

fn create_symble_table<'a>(commands: &'a [Command]) -> HashMap<&'a str, u16> {
    let mut symbol_table: HashMap<&str, u16> = HashMap::new();

    symbol_table = add_buildin_symbols(symbol_table);

    let mut current_line_num = 0;
    for command in commands {
        match command {
            Command::Loop(v) => {
                symbol_table.insert(v, current_line_num);
            }
            Command::Comment(_) => (),
            Command::Argument(_) | Command::Control(_) => current_line_num += 1,
        }
    }
    symbol_table
}

fn add_buildin_symbols(mut symbol_table: HashMap<&str, u16>) -> HashMap<&str, u16> {
    symbol_table.insert("SP", 0);
    symbol_table.insert("LCL", 1);
    symbol_table.insert("ARG", 2);
    symbol_table.insert("THIS", 3);
    symbol_table.insert("THAT", 4);
    symbol_table.insert("R0", 0);
    symbol_table.insert("R1", 1);
    symbol_table.insert("R2", 2);
    symbol_table.insert("R3", 3);
    symbol_table.insert("R4", 4);
    symbol_table.insert("R5", 5);
    symbol_table.insert("R6", 6);
    symbol_table.insert("R7", 7);
    symbol_table.insert("R8", 8);
    symbol_table.insert("R9", 9);
    symbol_table.insert("R10", 10);
    symbol_table.insert("R11", 11);
    symbol_table.insert("R12", 12);
    symbol_table.insert("R13", 13);
    symbol_table.insert("R14", 14);
    symbol_table.insert("R15", 15);
    symbol_table.insert("SCREEN", 0x4000);
    symbol_table.insert("KBD", 0x6000);
    symbol_table
}

fn parse_a_command(symbol_name: &str, symbol_table: &HashMap<&str, u16>) -> u16 {
    match symbol_name {
        v if v.parse::<u16>().is_ok() => v.parse::<u16>().unwrap(),
        v if symbol_table.contains_key(v) => *(symbol_table.get(v).unwrap()),
        _ => 1,
    }
}

fn parse_c_command(command: &str) -> String {
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
    let command = match command.find('=') {
        Some(v) => command.split_at(v + 1).1,
        _ => command,
    };
    let command = match command.find(';') {
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
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" => "0000000",
        "D|A" => "0010101",
        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" => "1110111",
        "M-1" => "1110010",
        "D+M" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "D|M" => "1010101",
        _ => "",
    };
    format!("111{comp}{dest}{jump}")
}
