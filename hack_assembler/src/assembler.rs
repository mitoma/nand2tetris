use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

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

    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap().trim().to_owned())
        .collect();
    let mut symbol_table: HashMap<&str, u16> = HashMap::new();

    // ここで symbol_table を作る予定
    let mut current_line_num = 0;
    for line in &lines {
        match &line {
            v if v.starts_with("(") && v.ends_with(")") => {
                let v = v.trim_matches(&['(', ')'] as &[_]);
                symbol_table.insert(&v, current_line_num);
            }
            v if v.starts_with("//") => (),
            v if v.is_empty() => (),
            _ => {
                current_line_num += 1;
                ()
            }
        }
    }

    let mut current_symbol_address = 1024;
    for line in &lines {
        let asm_line = match &line {
            v if v.starts_with("//") => None, // コメント
            v if v.starts_with("@") => {
                let symbol_name = &v[1..];
                if !symbol_table.contains_key(symbol_name) {
                    symbol_table.insert(symbol_name, current_symbol_address);
                    current_symbol_address += 1;
                }
                Some(format!(
                    "{:016b}",
                    parse_a_command(symbol_name, &symbol_table)
                ))
            }
            v if v.starts_with("(") && v.ends_with(")") => None,
            v if v.is_empty() => None,
            v => Some(parse_c_command(v)),
        };
        asm_line.map(|l| println!("{}", l));
    }
}

fn parse_a_command(symbol_name: &str, symbol_table: &HashMap<&str, u16>) -> u16 {
    match symbol_name {
        v if v.parse::<u16>().is_ok() => v.parse::<u16>().unwrap(),
        v if symbol_table.contains_key(v) => *(symbol_table.get(v).unwrap()),
        "SP" => 0,
        "LCL" => 1,
        "ARG" => 2,
        "THIS" => 3,
        "R0" => 0,
        "R1" => 1,
        "R2" => 2,
        "R3" => 3,
        "R4" => 4,
        "R5" => 5,
        "R6" => 6,
        "R7" => 7,
        "R8" => 8,
        "R9" => 9,
        "R10" => 10,
        "R11" => 11,
        "R12" => 12,
        "R13" => 13,
        "R14" => 14,
        "R15" => 15,
        "SCREEN" => 0x4000, // このへんまだ適当
        "KBD" => 0x6000,
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
        "M-1" => "1110010",
        "D+M" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "D|M" => "1010101",
        _ => "",
    };
    format!("111{}{}{}", comp, dest, jump)
}
