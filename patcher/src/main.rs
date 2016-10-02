extern crate regex;
extern crate rustc_demangle;

mod dol;
mod assembler;

use std::fs::File;
use std::io::{BufWriter, BufReader};
use std::io::prelude::*;
use assembler::Assembler;
use regex::Regex;
use rustc_demangle::demangle;
use dol::DolFile;
use assembler::Instruction;
use std::env::args;

const FRAMEWORK_MAP: &'static str = include_str!("../framework.map");
const HEADER: &'static str = r".text section layout
  Starting        Virtual
  address  Size   address
  -----------------------";

fn create_framework_map() {
    let regex = Regex::new(r".text.(.+)\s*\n*\s*0x(\w+)\s*\n*\s*0x(\w+)\s*\n*\s*.+\((.+)\)")
        .unwrap();
    let end_removal = Regex::new(r"^(.+E)\.?\d*$").unwrap();

    let mut file = BufReader::new(File::open("../../target/intermediate.elf.map").unwrap());
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut file = BufWriter::new(File::create("../../target/framework.map").unwrap());

    writeln!(file, "{}", HEADER).unwrap();

    for captures in regex.captures_iter(&content) {
        let mangled = captures.at(1).unwrap().trim();
        let mangled = end_removal.captures(mangled).map_or(mangled, |c| c.at(1).unwrap());
        let fn_name = demangle(mangled).to_string();
        let address = captures.at(2).unwrap();
        let length = captures.at(3).unwrap();
        let source_file = captures.at(4).unwrap();
        let length = u32::from_str_radix(length, 16).unwrap();

        let mut fn_name: &str = &fn_name.replace(' ', "_")
            .replace("()", "Void")
            .replace("(", "Tuple<")
            .replace(")", ">");

        let fn_name_bytes = fn_name.as_bytes();

        if fn_name.len() >= 19 && &fn_name_bytes[fn_name.len() - 19..][..3] == b"::h" {
            fn_name = &fn_name[..fn_name.len() - 19];
        }

        if address != "00000000" {
            writeln!(file,
                     "  00000000 {:06x} {}  4 {} \t{}",
                     length,
                     address,
                     fn_name,
                     source_file)
                .unwrap();
        }
    }

    write!(file, "{}", FRAMEWORK_MAP).unwrap();
}

fn main() {
    let mut asm = String::new();
    let _ = File::open("../../src/patch.asm")
        .expect("Couldn't find \"src/patch.asm\". If you don't need to patch the dol, just \
                 create an empty file.")
        .read_to_string(&mut asm);

    let lines = &asm.lines().collect::<Vec<_>>();

    let mut assembler = Assembler::new("../../target/intermediate.elf");
    let instructions = &assembler.assemble_all_lines(lines);

    let mut intermediate = Vec::new();
    let _ = File::open("../../target/intermediate.dol")
        .expect("Couldn't find \"target/intermediate.dol\". Did you build the project correctly \
                 using \"make\"?")
        .read_to_end(&mut intermediate);

    let intermediate = DolFile::new(&intermediate);

    if let Some("cheat") = args().skip(1).next().as_ref().map(|x| x as &str) {
        write_cheat(intermediate, instructions);
    } else {
        let mut original = Vec::new();
        let _ = File::open("../../game/original.dol")
            .expect("Couldn't find \"game/original.dol\". You need to copy the game's main.dol \
                     there.")
            .read_to_end(&mut original);


        let original = DolFile::new(&original);
        patch_game(original, intermediate, instructions);
    }
}

fn patch_game(original: DolFile, intermediate: DolFile, instructions: &[Instruction]) {
    let mut original = original;

    original.append(intermediate);
    original.patch(instructions);

    let data = original.to_bytes();
    let mut file = File::create("../../game/sys/main.dol")
        .expect("Couldn't create \"game/sys/main.dol\". You might need to provide higher \
                 privileges.");

    file.write(&data).expect("Couldn't write the main.dol");

    create_framework_map();
}

fn write_cheat(intermediate: DolFile, instructions: &[Instruction]) {
    let mut file = File::create("../../cheat.txt")
        .expect("Couldn't create \"cheat.txt\". You might need to provide higher \
                 privileges.");

    writeln!(file, "A8000000 00000001").unwrap();

    for instruction in instructions {
        writeln!(file,
                 "{:08X} {:08X}",
                 (instruction.address & 0x01FFFFFF) | 0x04000000,
                 instruction.data)
            .unwrap();
    }

    for section in intermediate.text_sections.iter().chain(intermediate.data_sections.iter()) {
        writeln!(file,
                 "{:08X} {:08X}",
                 (section.address & 0x01FFFFFF) | 0x06000000,
                 section.data.len())
            .unwrap();
        let line_ender = if section.data.len() % 8 > 0 {
            8 - (section.data.len() % 8)
        } else {
            0
        };
        for (i, byte) in section.data
            .iter()
            .chain(std::iter::repeat(&0).take(line_ender))
            .enumerate() {
            if i % 8 == 4 {
                write!(file, " ").unwrap();
            }

            write!(file, "{:02X}", byte).unwrap();

            if i % 8 == 7 {
                writeln!(file, "").unwrap();
            }
        }
    }

    // for section in intermediate.text_sections.iter().chain(intermediate.data_sections.iter()) {
    //     let mut address = section.address;

    //     let line_ender = if section.data.len() % 4 > 0 {
    //         4 - (section.data.len() % 4)
    //     } else {
    //         0
    //     };

    //     for (i, byte) in section.data.iter().chain(std::iter::repeat(&0).take(line_ender)).enumerate() {
    //         if i % 4 == 0 {
    //             write!(file, "{:08X} ", (address & 0x01FFFFFF) | 0x04000000).unwrap();
    //         }

    //         write!(file, "{:02X}", byte).unwrap();

    //         if i % 4 == 3 {
    //             writeln!(file, "").unwrap();
    //         }

    //         address += 1;
    //     }
    // }
}
