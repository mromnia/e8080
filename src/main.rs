mod disassembler;
mod opcode_decoder;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut opcode_data = String::new();
    let mut rom_data: Vec<u8> = Vec::new();

    {
        let mut opcode_file = File::open("./data/opcodes.txt").unwrap();
        opcode_file.read_to_string(&mut opcode_data).unwrap();
    }

    let decoder = opcode_decoder::OpcodeDecoder::new(&opcode_data);

    {
        let mut rom_file = File::open("./data/invaders.rom").unwrap();
        rom_file.read_to_end(&mut rom_data).unwrap();
    }

    let da = disassembler::Disassembler::new(decoder);

    let ops = da.disassemble(&rom_data);

    for (idx, op) in ops.into_iter().enumerate() {
        if idx > 20 {
            return;
        }

        if let Some(optype) = op.optype.upgrade() {
            match op {
                opcode_decoder::Op {
                    arg1: Some(x),
                    arg2: Some(y),
                    ..
                } => println!("{} {:x?} {:x?}", optype.instruction, x, y),
                opcode_decoder::Op { arg1: Some(x), .. } => {
                    println!("{} {:x?}", optype.instruction, x)
                }
                _ => println!("{}", optype.instruction),
            }
        }
    }
}
