mod disassembler;
mod emulator;
mod opcode_decoder;

use std::fs::File;
use std::io;
use std::io::prelude::*;

static DISASSEMBLE: bool = false;

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

    if DISASSEMBLE {
        let da = disassembler::Disassembler::new(decoder);

        let ops = da.disassemble(&rom_data);

        for op in ops.into_iter() {
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
    } else {
        let mut cpu = emulator::CPU::new(decoder);
        cpu.set_memory(0, &rom_data);

        loop {
            let mut cmd = String::new();
            io::stdin().read_line(&mut cmd).unwrap();

            let mut ticks = 1;

            if cmd.trim().len() > 0 {
                ticks = cmd.trim().parse().unwrap();
            }

            loop {
                cpu.tick();
                ticks -= 1;

                if ticks == 0 {
                    break;
                }
            }

            cpu.print_state();
        }
    }
}
