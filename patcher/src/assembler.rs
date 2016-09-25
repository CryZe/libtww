use std::i64;
use std::u32;
use std::num::ParseIntError;
use std::collections::HashMap;
use std::process::Command;
use std::str;

pub struct Assembler {
    symbol_table: HashMap<String, u32>,
    program_counter: u32,
}

pub struct Instruction {
    pub address: u32,
    pub data: u32,
}

impl Assembler {
    pub fn new(elf_path: &str) -> Assembler {
        Assembler {
            symbol_table: build_symbol_table(elf_path),
            program_counter: 0,
        }
    }

    pub fn assemble_all_lines(&mut self, lines: &[&str]) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let filtered_lines = lines.iter().map(|l| reduce_line_to_code(l)).filter(|l| !l.is_empty());

        for line in filtered_lines {
            if line.ends_with(':') {
                self.program_counter = parse_program_counter_label(line)
                    .expect("Couldn't parse Address Label");
            } else {
                let instruction = self.parse_instruction(line);
                instructions.push(instruction);
                self.program_counter += 4;
            }
        }

        instructions
    }

    fn parse_instruction(&self, line: &str) -> Instruction {
        let data;

        if line.starts_with("bl ") {
            let operand = &line[3..];
            let destination = self.resolve_symbol(operand);
            data = build_branch_instruction(self.program_counter, destination, false, true);
        } else if line.starts_with("b ") {
            let operand = &line[2..];
            let destination = self.resolve_symbol(operand);
            data = build_branch_instruction(self.program_counter, destination, false, false);
        } else if line == "nop" {
            data = 0x60000000;
        } else {
            panic!("Unknown instruction: {}", line);
        }

        Instruction {
            address: self.program_counter,
            data: data,
        }
    }

    fn resolve_symbol(&self, symbol: &str) -> u32 {
        parse_u32_literal(symbol).unwrap_or_else(|_| {
            *self.symbol_table
                .get(symbol)
                .unwrap_or_else(|| panic!("The symbol \"{}\" wasn't found", symbol))
        })
    }
}

fn reduce_line_to_code(line: &str) -> &str {
    let mut line = line;
    if let Some(index) = line.find(';') {
        line = &line[..index];
    }
    line.trim()
}

fn parse_i64_literal(literal: &str) -> Result<i64, ParseIntError> {
    let mut literal = literal;
    if let Some(index) = literal.find("0x") {
        let negative = literal.starts_with('-');
        literal = &literal[index + 2..];

        let factor = if negative { -1 } else { 1 };

        i64::from_str_radix(literal, 16).map(|i| factor * i)
    } else {
        literal.parse()
    }
}

fn parse_u32_literal(literal: &str) -> Result<u32, ParseIntError> {
    parse_i64_literal(literal).map(|i| i as u32)
}

fn parse_program_counter_label(line: &str) -> Result<u32, ParseIntError> {
    let line = &line[..line.len() - 1];
    parse_u32_literal(line)
}

fn build_symbol_table(elf_path: &str) -> HashMap<String, u32> {
    let output = Command::new("powerpc-eabi-nm")
        .arg(elf_path)
        .output()
        .expect("Failed to retrieve the symbol table");

    let output = str::from_utf8(&output.stdout)
        .expect("The symbol table is not a proper UTF-8 string");

    let mut symbol_table = HashMap::new();

    for line in output.lines() {
        let mut split = line.split_whitespace();
        let address = split.next().expect("The symbol table isn't properly formatted");

        let address = if let Ok(address) = u32::from_str_radix(address, 16) {
            address
        } else {
            continue;
        };

        let symbol = split.nth(1).expect("The symbol table isn't properly formatted");

        symbol_table.insert(symbol.to_owned(), address);
    }

    symbol_table
}

fn build_branch_instruction(address: u32, destination: u32, aa: bool, lk: bool) -> u32 {
    let bits_dest = if aa {
        destination
    } else {
        destination - address
    };
    let bits_aa = if aa { 1 } else { 0 };
    let bits_lk = if lk { 1 } else { 0 };

    (18 << 26) | (0x3FFFFFC & bits_dest) | (bits_aa << 1) | bits_lk
}
