extern crate e8080;

mod renderer;

use e8080::*;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Ok(i) = args.binary_search(&String::from("--disassemble")) {
        if args.len() < (i + 2) {
            println!("{}", "Required argument: file to disassemble");
            ::std::process::exit(1);
        }

        disassemble(&args[i + 1]);
    } else if let Ok(_) = args.binary_search(&String::from("--cpu-diag")) {
        run_cpu_diag();
    } else {
        run_game();
    }
}

fn run_game() {
    let decoder = e8080::decoder_new();
    let am: *mut e8080::emulator::ArcadeMachine = e8080::am_new(decoder);
    e8080::am_get_render_buffer(am);
    renderer::run(am);
}

fn run_cpu_diag() {
    use emulator::cpu::CPU;

    let opcode_data = load_opcodes();
    let decoder = opcode_decoder::OpcodeDecoder::new(&opcode_data);
    let cpudiag = load_cpudiag();

    let mut cpu = emulator::cpu::CPU::new(decoder);
    cpu.debug = true;
    cpu.set_memory(0x0100, &cpudiag); // the rom expects to be loaded at 0x0100
    cpu.set_memory(368, &[0x7]); // fix a bug, supposedly

    cpu.set_memory(0x00, &[0xc3, 0x00, 0x01]); // JMP 0x0100

    // A assembly subroutine to set D in OUT 0 and E in OUT 1 to print a message
    #[cfg_attr(rustfmt, rustfmt_skip)]
    cpu.set_memory(0x0005, &[
        0xf5,               // PUSH PSW
        0x3e, 0x09,         // MVI A, 9
        0xb9,               // CMP C
        0xca, 0x10, 0x00,   // JZ
        0x3e, 0x02,         // MVI A, 2
        0xb9,               // CMP C
        0xc0,               // RNZ
        0x7a,               // MOV A, D
        0xd3, 0x00,         // OUT 0
        0x7b,               // MOV A, E
        0xd3, 0x01,         // OUT 1
        0xf1,               // POP PSW
        0xc9                // RET
    ]);

    fn msg_printer(cpu: &mut CPU) -> &mut CPU {
        if cpu.pc() == 0 {
            println!("{}", "Success!");
            ::std::process::exit(0);
        }

        let (val1, dirty1) = cpu.get_out_port(1);

        if !dirty1 {
            return cpu;
        }

        let (val0, _) = cpu.get_out_port(0);

        let mut offset = ((val0 as u16) << 8) | (val1 as u16);
        offset += 4;

        let mut s = std::char::from_u32(cpu.get_memory(offset) as u32)
            .unwrap()
            .to_string();

        loop {
            let c = std::char::from_u32(cpu.get_memory(offset) as u32).unwrap();
            if c == '$' {
                break;
            }

            s += &c.to_string();
            offset += 1;
        }

        println!("{}", s);

        if s.contains("FAILED") {
            ::std::process::exit(1);
        }

        cpu
    };

    step_cpu(&mut cpu, msg_printer);
}

fn step_cpu(
    cpu: &mut emulator::cpu::CPU,
    step_callback: fn(&mut emulator::cpu::CPU) -> &mut emulator::cpu::CPU,
) {
    let mut cpu = cpu;
    let mut instr: u64 = 0;

    loop {
        let mut ticks = 1;

        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();

        if cmd.trim().len() > 0 {
            ticks = cmd.trim().parse().unwrap();
        }

        loop {
            print!("{}: ", instr);

            cpu.tick();
            step_callback(&mut cpu);
            ticks -= 1;
            instr += 1;

            if ticks == 0 {
                break;
            }
        }

        cpu.print_state();
    }
}

fn disassemble(file: &str) {
    let opcode_data = load_opcodes();
    let decoder = opcode_decoder::OpcodeDecoder::new(&opcode_data);

    let data = load_binary_file(file);

    let partial_data = if file.contains("cpudiag.bin") {
        &data[0x0100..]
    } else {
        &data
    };

    let da = disassembler::Disassembler::new(decoder);

    let ops = da.disassemble(&partial_data);

    for op in ops.into_iter() {
        println!("{}", op.to_string());
    }
}

fn load_opcodes() -> String {
    let mut opcode_data = String::new();
    let mut opcode_file = File::open("./data/opcodes.txt").unwrap();
    opcode_file.read_to_string(&mut opcode_data).unwrap();
    opcode_data
}

fn load_binary_file(path: &str) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    let mut file = File::open(path).unwrap();
    file.read_to_end(&mut data).unwrap();
    data
}

fn load_invaders() -> Vec<u8> {
    load_binary_file("./data/invaders.rom")
}

fn load_cpudiag() -> Vec<u8> {
    load_binary_file("./data/cpudiag.bin")
}
