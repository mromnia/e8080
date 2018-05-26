mod disassembler;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("./data/invaders.rom").unwrap();

    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let da = disassembler::Disassembler::new("./data/opcodes.txt");

    let ops = da.disassemble(&contents);

    for (idx, op) in ops.into_iter().enumerate() {
        if let Some(optype) = op.optype.upgrade() {
            match op {
                disassembler::Op { arg1: Some(x), arg2: Some(y), .. } => println!("{} {} {}", optype.instruction, x, y),
                disassembler::Op { arg1: Some(x), .. } => println!("{} {}", optype.instruction, x),
                _ => println!("{}", optype.instruction),
            }
        }
    }
}
