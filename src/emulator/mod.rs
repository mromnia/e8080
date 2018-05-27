mod math;
mod ops;

use opcode_decoder::*;
use std::boxed::Box;

const MEMORY_SIZE: usize = 65536;

#[derive(Copy, Clone)]
enum Flag {
    S,
    Z,
    AC,
    P,
    C,
}

impl Flag {
    pub fn bit(&self) -> u8 {
        match self {
            Flag::S => 0x01,
            Flag::Z => 0x02,
            Flag::AC => 0x08,
            Flag::P => 0x20,
            Flag::C => 0x80,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    Memory,
}

impl Register {
    pub fn by_code(code: u8) -> Register {
        match code & 0x07 {
            0x00 => Register::B,
            0x01 => Register::C,
            0x02 => Register::D,
            0x03 => Register::E,
            0x04 => Register::H,
            0x05 => Register::L,
            0x06 => Register::Memory,
            0x07 => Register::A,
            _ => panic!("Invalid register code"),
        }
    }

    pub fn pair_by_code(code: u8) -> (Register, Register) {
        match code & 0x03 {
            0x00 => (Register::B, Register::C),
            0x01 => (Register::D, Register::E),
            0x02 => (Register::H, Register::L),
            _ => panic!("Invalid register code"),
        }
    }
}

pub struct CPU {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    flags: Flags,
    enable_interrupts: bool,
    memory: Memory,

    decoder: OpcodeDecoder,
}

struct Flags {
    flags: u8,
}

struct Memory {
    data: Box<[u8; MEMORY_SIZE]>,
}

impl CPU {
    pub fn new(decoder: OpcodeDecoder) -> CPU {
        CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            flags: Flags { flags: 0 },
            enable_interrupts: false,
            memory: Memory::new(),

            decoder,
        }
    }

    pub fn set_memory(&mut self, addr: u16, data: &[u8]) {
        self.memory.set_block(addr, data);
    }

    pub fn tick(&mut self) {
        let (op, len) = self.decoder
            .get_next_op(self.memory.get_to_end(self.pc))
            .unwrap();

        self.execute_op(&op, len);
    }

    pub fn update_flags(&mut self, result: u8, carry: Option<bool>, acarry: Option<bool>) {
        self.update_s(result);
        self.update_z(result);
        self.update_p(result);

        if let Some(c) = carry {
            self.flags.set(Flag::C, c);
        }

        if let Some(ac) = acarry {
            self.flags.set(Flag::AC, ac);
        }
    }

    fn update_s(&mut self, result: u8) {
        self.flags.set(Flag::S, (result & 0x80) > 0);
    }

    fn update_z(&mut self, result: u8) {
        self.flags.set(Flag::Z, result == 0);
    }

    fn update_p(&mut self, result: u8) {
        self.flags.set(Flag::P, result.count_ones() % 2 == 0);
    }

    fn get_reg(&self, code: Register) -> &u8 {
        match code {
            Register::A => &self.a,
            Register::B => &self.b,
            Register::C => &self.c,
            Register::D => &self.d,
            Register::E => &self.e,
            Register::H => &self.h,
            Register::L => &self.l,
            _ => panic!("Attempt to get memory as register"),
        }
    }

    fn get_reg_mut(&mut self, code: Register) -> &mut u8 {
        match code {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
            Register::E => &mut self.e,
            Register::H => &mut self.h,
            Register::L => &mut self.l,
            _ => panic!("Attempt to get memory as register"),
        }
    }

    fn get_reg_value(&self, code: Register) -> u8 {
        match code {
            Register::Memory => self.memory.get(math::combine_8_to_16(self.h, self.l)),
            _ => *self.get_reg(code),
        }
    }

    fn set_reg_value(&mut self, code: Register, val: u8) {
        match code {
            Register::Memory => self.memory.set(math::combine_8_to_16(self.h, self.l), val),
            _ => *self.get_reg_mut(code) = val,
        };
    }

    fn get_reg_pair_value(&self, code1: Register, code2: Register) -> u16 {
        let val1 = self.get_reg_value(code1);
        let val2 = self.get_reg_value(code2);
        math::combine_8_to_16(val1, val2)
    }

    fn set_reg_pair_value(&mut self, code1: Register, code2: Register, val: u16) {
        self.set_reg_value(code2, math::lower_8(val));
        self.set_reg_value(code1, math::higher_8(val));
    }

    fn reg_add(&mut self, code: Register, val: u8, with_carry: bool) {
        let old_val = self.get_reg_value(code);

        let carry_val = if with_carry && self.flags.is_set(Flag::C) {
            1
        } else {
            0
        };

        let (result, carry, acarry) = math::add_8(old_val, val + carry_val);
        self.set_reg_value(code, result);
        self.update_flags(result, Some(carry), Some(acarry));
    }

    fn reg_sub(&mut self, code: Register, val: u8, with_carry: bool) {
        let old_val = self.get_reg_value(code);

        let carry_val = if with_carry && self.flags.is_set(Flag::C) {
            1
        } else {
            0
        };

        let (result, carry, acarry) = math::sub_8(old_val, val + carry_val);
        self.set_reg_value(code, result);
        self.update_flags(result, Some(carry), Some(acarry));
    }

    fn reg_and(&mut self, code: Register, val: u8) {
        let old_val = self.get_reg_value(code);
        let result = old_val & val;
        self.set_reg_value(code, result);
        self.update_flags(result, Some(false), None);
    }

    fn reg_xor(&mut self, code: Register, val: u8) {
        let old_val = self.get_reg_value(code);
        let result = old_val ^ val;
        self.set_reg_value(code, result);
        self.update_flags(result, Some(false), None);
    }

    fn reg_or(&mut self, code: Register, val: u8) {
        let old_val = self.get_reg_value(code);
        let result = old_val | val;
        self.set_reg_value(code, result);
        self.update_flags(result, Some(false), None);
    }

    fn reg_cmp(&mut self, code: Register, val: u8) {
        let old_val = self.get_reg_value(code);
        let (result, carry, acarry) = math::sub_8(old_val, val);
        self.set_reg_value(code, result);
        self.update_flags(result, Some(carry), Some(acarry));
    }

    fn reg_pair_add(&mut self, code1: Register, code2: Register, val: u16, set_carry: bool) {
        let old_val = self.get_reg_pair_value(code1, code2);

        let (result, carry) = math::add_16(old_val, val);

        self.set_reg_pair_value(code1, code2, result);

        if set_carry {
            self.flags.set(Flag::C, carry);
        }
    }
}

impl Flags {
    pub fn is_set(&self, flag: Flag) -> bool {
        self.flags & flag.bit() > 0
    }

    pub fn set(&mut self, flag: Flag, toggle: bool) {
        if toggle {
            self.flags = self.flags | flag.bit();
        } else {
            self.flags = self.flags & !flag.bit();
        }
    }

    pub fn flip(&mut self, flag: Flag) {
        let val = self.is_set(flag);
        self.set(flag, !val);
    }
}

impl Memory {
    pub fn new() -> Memory {
        let data = Box::new([0; MEMORY_SIZE]);

        Memory { data }
    }

    pub fn set(&mut self, addr: u16, data: u8) {
        let addr = addr as usize;
        self.data[addr] = data;
    }

    pub fn get(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        self.data[addr]
    }

    pub fn set_block(&mut self, addr: u16, data: &[u8]) {
        let mut addr = addr;

        for d in data {
            self.set(addr, *d);
            addr += 1;
        }
    }

    pub fn get_to_end(&self, addr: u16) -> &[u8] {
        let addr = addr as usize;
        &self.data[addr..]
    }
}