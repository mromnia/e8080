extern crate rand;

use super::test::rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;

use super::*;

fn init_decoder() -> OpcodeDecoder {
    let mut opcode_data = String::new();
    {
        let mut opcode_file = File::open("./data/opcodes.txt").unwrap();
        opcode_file.read_to_string(&mut opcode_data).unwrap();
    }
    OpcodeDecoder::new(&opcode_data)
}

fn set_op_at_rnd_addr(cpu: &mut CPU, op: u8) -> u16 {
    let addr: u16 = random();
    let addr = addr & 0x1fff;

    cpu.set_memory(addr, &[op]);
    cpu.pc = addr;

    addr
}

#[test]
fn test_nop() {
    let mut cpu = CPU::new(init_decoder());

    let addr = set_op_at_rnd_addr(&mut cpu, 0x00);
    cpu.tick();

    assert_eq!(cpu.pc, addr + 1);
}

#[test]
fn test_inr() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x04);
    cpu.b = -4i8 as u8;
    cpu.tick();

    assert_eq!(cpu.b as i8, -3);
    assert!(cpu.flags.is_set(Flag::S));
    assert!(!cpu.flags.is_set(Flag::Z));
    assert!(!cpu.flags.is_set(Flag::C));
    assert!(!cpu.flags.is_set(Flag::P));
}

#[test]
fn test_dcr() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x0d);
    cpu.c = 64;
    cpu.tick();

    assert_eq!(cpu.c as i8, 63);
    assert!(!cpu.flags.is_set(Flag::S));
    assert!(!cpu.flags.is_set(Flag::Z));
    assert!(!cpu.flags.is_set(Flag::C));
    assert!(cpu.flags.is_set(Flag::P));
}

#[test]
fn test_add() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x80);
    cpu.a = 3;
    cpu.b = 200;
    cpu.tick();

    assert_eq!(cpu.a, 203);
    assert!(cpu.flags.is_set(Flag::S));
    assert!(!cpu.flags.is_set(Flag::Z));
    assert!(!cpu.flags.is_set(Flag::C));
    assert!(!cpu.flags.is_set(Flag::P));
}

#[test]
fn test_add_mem() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x86);
    cpu.a = 3;

    let mem_addr = 0xf000;
    cpu.memory.set(mem_addr, 5);
    cpu.h = math::higher_8(mem_addr);
    cpu.l = math::lower_8(mem_addr);
    cpu.tick();

    assert_eq!(cpu.a, 8);
    assert!(!cpu.flags.is_set(Flag::S));
    assert!(!cpu.flags.is_set(Flag::Z));
    assert!(!cpu.flags.is_set(Flag::C));
    assert!(!cpu.flags.is_set(Flag::P));
}

#[test]
fn test_adc() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x88);
    cpu.a = 3;
    cpu.b = 200;
    cpu.flags.set(Flag::C, true);
    cpu.tick();

    assert_eq!(cpu.a, 204);
    assert!(cpu.flags.is_set(Flag::S));
    assert!(!cpu.flags.is_set(Flag::Z));
    assert!(!cpu.flags.is_set(Flag::C));
    assert!(cpu.flags.is_set(Flag::P));
}

#[test]
fn test_sub() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x94);
    cpu.a = 5;
    cpu.h = 10;
    cpu.tick();

    assert_eq!(cpu.a as i8, -5);
    assert!(cpu.flags.is_set(Flag::S));
    assert!(!cpu.flags.is_set(Flag::Z));
    assert!(cpu.flags.is_set(Flag::C));
    assert!(!cpu.flags.is_set(Flag::P));
}

#[test]
fn test_sub_a() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x97);
    cpu.a = 5;
    cpu.tick();

    assert_eq!(cpu.a, 0);
    assert!(!cpu.flags.is_set(Flag::S));
    assert!(cpu.flags.is_set(Flag::Z));
    assert!(!cpu.flags.is_set(Flag::C));
    assert!(cpu.flags.is_set(Flag::P));
}

#[test]
fn test_sbb() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x9c);
    cpu.a = 5;
    cpu.h = 10;
    cpu.flags.set(Flag::C, true);
    cpu.tick();

    assert_eq!(cpu.a as i8, -6);
    assert!(cpu.flags.is_set(Flag::S));
    assert!(!cpu.flags.is_set(Flag::Z));
    assert!(cpu.flags.is_set(Flag::C));
    assert!(cpu.flags.is_set(Flag::P));
}

#[test]
fn test_ana() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0xa1);
    cpu.a = 0b11111100;
    cpu.c = 0b00001111;
    cpu.tick();

    assert_eq!(cpu.a, 0b00001100);
    assert!(!cpu.flags.is_set(Flag::S));
    assert!(!cpu.flags.is_set(Flag::Z));
    assert!(!cpu.flags.is_set(Flag::C));
    assert!(cpu.flags.is_set(Flag::P));
}

#[test]
fn test_xra() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0xa8);
    cpu.a = 0b01011100;
    cpu.b = 0b01111000;
    cpu.tick();

    assert_eq!(cpu.a, 0b00100100);
    assert!(!cpu.flags.is_set(Flag::S));
    assert!(!cpu.flags.is_set(Flag::Z));
    assert!(!cpu.flags.is_set(Flag::C));
    assert!(cpu.flags.is_set(Flag::P));
}

#[test]
fn test_ora() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0xb1);
    cpu.a = 0b00110011;
    cpu.c = 0b00001111;
    cpu.tick();

    assert_eq!(cpu.a, 0b00111111);
    assert!(!cpu.flags.is_set(Flag::S));
    assert!(!cpu.flags.is_set(Flag::Z));
    assert!(!cpu.flags.is_set(Flag::C));
    assert!(cpu.flags.is_set(Flag::P));
}

#[test]
fn test_mov() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x6a);
    cpu.l = 5;
    cpu.d = 83;
    cpu.tick();

    assert_eq!(cpu.l, 83);
}

#[test]
fn test_mov_from_mem() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x5e);
    cpu.e = 5;

    let mem_addr = 0xf0a0;
    cpu.memory.set(mem_addr, 12);
    cpu.h = math::higher_8(mem_addr);
    cpu.l = math::lower_8(mem_addr);
    cpu.tick();

    assert_eq!(cpu.e, 12);
}

#[test]
fn test_mov_to_mem() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x74);
    let mem_addr = 0xf0a0;
    cpu.memory.set(mem_addr, 12);
    cpu.h = math::higher_8(mem_addr);
    cpu.l = math::lower_8(mem_addr);
    cpu.tick();

    assert_eq!(cpu.memory.get(mem_addr), 0xf0);
}

#[test]
fn test_rlc() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x07);
    cpu.a = 0b10110010;
    cpu.flags.set(Flag::C, true);
    cpu.tick();

    assert_eq!(cpu.a, 0b01100100);
    assert!(cpu.flags.is_set(Flag::C));
}

#[test]
fn test_ral() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x17);
    cpu.a = 0b00110010;
    cpu.flags.set(Flag::C, true);
    cpu.tick();

    assert_eq!(cpu.a, 0b01100101);
    assert!(!cpu.flags.is_set(Flag::C));
}

#[test]
fn test_ral_no_carry() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x17);
    cpu.a = 0b10110010;
    cpu.tick();

    assert_eq!(cpu.a, 0b01100100);
    assert!(cpu.flags.is_set(Flag::C));
}

#[test]
fn test_rrc() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x0f);
    cpu.a = 0b00110011;
    cpu.flags.set(Flag::C, true);
    cpu.tick();

    assert_eq!(cpu.a, 0b00011001);
    assert!(cpu.flags.is_set(Flag::C));
}

#[test]
fn test_rar() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x1f);
    cpu.a = 0b10110010;
    cpu.flags.set(Flag::C, true);
    cpu.tick();

    assert_eq!(cpu.a, 0b11011001);
    assert!(!cpu.flags.is_set(Flag::C));
}

#[test]
fn test_rar_no_carry() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x1f);
    cpu.a = 0b10110011;
    cpu.tick();

    assert_eq!(cpu.a, 0b01011001);
    assert!(cpu.flags.is_set(Flag::C));
}

#[test]
fn test_inx() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x03);
    cpu.b = 2i8 as u8;
    cpu.c = -1i8 as u8;
    cpu.tick();

    assert_eq!(cpu.b as i8, 3);
    assert_eq!(cpu.c as i8, 0);
}

#[test]
fn test_dcx() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x1b);
    cpu.d = 4;
    cpu.e = 0;
    cpu.tick();

    assert_eq!(cpu.d, 3);
    assert_eq!(cpu.e, 255);
}

#[test]
fn test_dad() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x09);
    cpu.b = 4;
    cpu.c = 255;
    cpu.h = 4;
    cpu.l = 1;
    cpu.tick();

    assert_eq!(cpu.b, 4);
    assert_eq!(cpu.c, 255);
    assert_eq!(cpu.h, 9);
    assert_eq!(cpu.l, 0);
    assert!(!cpu.flags.is_set(Flag::C));
}

#[test]
fn test_dad_carry() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x09);
    cpu.b = 255;
    cpu.c = 255;
    cpu.h = 0;
    cpu.l = 1;
    cpu.tick();

    assert_eq!(cpu.b, 255);
    assert_eq!(cpu.c, 255);
    assert_eq!(cpu.h, 0);
    assert_eq!(cpu.l, 0);
    assert!(cpu.flags.is_set(Flag::C));
}

#[test]
fn test_daa() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x27);
    cpu.a = 0b10011011;
    cpu.tick();

    assert_eq!(cpu.a, 1);
    assert!(cpu.flags.is_set(Flag::C));
    assert!(cpu.flags.is_set(Flag::AC));
}

#[test]
fn test_stax() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x02);
    cpu.a = 93;

    let mem_addr = 0xd033;
    cpu.memory.set(mem_addr, 33);
    cpu.b = math::higher_8(mem_addr);
    cpu.c = math::lower_8(mem_addr);
    cpu.tick();

    assert_eq!(cpu.memory.get(mem_addr), 93);
}

#[test]
fn ldax() {
    let mut cpu = CPU::new(init_decoder());

    set_op_at_rnd_addr(&mut cpu, 0x1a);
    cpu.a = 3;

    let mem_addr = 0xd011;
    cpu.memory.set(mem_addr, 54);
    cpu.d = math::higher_8(mem_addr);
    cpu.e = math::lower_8(mem_addr);
    cpu.tick();

    assert_eq!(cpu.a, 54);
}
