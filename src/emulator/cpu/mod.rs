mod flags;
mod ops;
mod port;

use std::boxed::Box;

use self::flags::{Flag, FlagRegister};
use self::port::{InPort, OutPort};
use super::math;
use opcode_decoder::*;

const MEMORY_SIZE: usize = 65536;
const PORT_NUM: usize = 8;

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
    S,
    P,
    Flags,
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
            0x03 => (Register::S, Register::P),
            _ => panic!("Invalid register code"),
        }
    }

    pub fn pair_by_code_pushpop(code: u8) -> (Register, Register) {
        match code & 0x03 {
            0x00 => (Register::B, Register::C),
            0x01 => (Register::D, Register::E),
            0x02 => (Register::H, Register::L),
            0x03 => (Register::Flags, Register::A),
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
    enable_interrupts: bool,

    flags: FlagRegister,
    memory: Memory,

    in_ports: Vec<InPort>,
    out_ports: Vec<OutPort>,

    decoder: OpcodeDecoder,
}

struct Memory {
    data: Box<[u8; MEMORY_SIZE]>,
}

impl CPU {
    pub fn new(decoder: OpcodeDecoder) -> CPU {
        let mut in_ports = Vec::new();
        let mut out_ports = Vec::new();

        for _i in 0..PORT_NUM {
            in_ports.push(InPort::new(0));
            out_ports.push(OutPort::new(0));
        }

        CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0xf000,
            pc: 0,
            flags: FlagRegister::new(),
            enable_interrupts: false,
            memory: Memory::new(),

            in_ports,
            out_ports,

            decoder,
        }
    }

    pub fn set_memory(&mut self, addr: u16, data: &[u8]) {
        self.memory.set_block(addr, data);
    }

    pub fn get_memory_to_end(&self, addr: u16) -> &[u8] {
        self.memory.get_to_end(addr)
    }

    pub fn tick(&mut self) -> u8 {
        let op = self
            .decoder
            .get_next_op(self.memory.get_to_end(self.pc))
            .unwrap();

        self.execute_op(&op)
    }

    pub fn print_state(&self) {
        println!("{}", &self.to_string());
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:#04x?} {:#04x?} {:#04x?} {:#04x?} {:#04x?} {:#04x?} {:#04x?}\n{:#010b} {:#06x?} {:#06x?}",
            self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.flags.get_all(), self.sp, self.pc
        )
    }

    fn update_flags(&mut self, result: u8, carry: Option<bool>, acarry: Option<bool>) {
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
            Register::S => math::higher_8(self.sp),
            Register::P => math::lower_8(self.sp),
            Register::Flags => self.flags.get_all(),
            _ => *self.get_reg(code),
        }
    }

    fn set_reg_value(&mut self, code: Register, val: u8) {
        match code {
            Register::Memory => self.memory.set(math::combine_8_to_16(self.h, self.l), val),
            Register::S => self.sp = (self.sp & 0x00FF) | ((val as u16) << 8),
            Register::P => self.sp = (self.sp & 0xFF00) | (val as u16),
            Register::Flags => self.flags.set_all(val),
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

    fn reg_add(&mut self, code: Register, val: u8, set_carry: bool, with_carry: bool) {
        let old_val = self.get_reg_value(code);

        let carry_val = if with_carry && self.flags.is_set(Flag::C) {
            1
        } else {
            0
        };

        let (result, carry, acarry) = math::add_8(old_val, val + carry_val);
        self.set_reg_value(code, result);
        let carry = if set_carry { Some(carry) } else { None };
        self.update_flags(result, carry, Some(acarry));
    }

    fn reg_sub(&mut self, code: Register, val: u8, set_carry: bool, with_carry: bool) {
        let old_val = self.get_reg_value(code);

        let carry_val = if with_carry && self.flags.is_set(Flag::C) {
            1
        } else {
            0
        };

        let (result, carry, acarry) = math::sub_8(old_val, val + carry_val);
        self.set_reg_value(code, result);
        let carry = if set_carry { Some(carry) } else { None };
        self.update_flags(result, carry, Some(acarry));
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

    fn reg_mov(&mut self, code1: Register, code2: Register) {
        let val = self.get_reg_value(code2);
        self.set_reg_value(code1, val);
    }

    fn reg_rot_left(&mut self, with_carry: bool) {
        let val = self.get_reg_value(Register::A);
        let (result, carry) = math::rot_left(val, with_carry && self.flags.is_set(Flag::C));
        self.set_reg_value(Register::A, result);
        self.update_flags(result, Some(carry), None);
    }

    fn reg_rot_right(&mut self, with_carry: bool) {
        let val = self.get_reg_value(Register::A);
        let (result, carry) = math::rot_right(val, with_carry && self.flags.is_set(Flag::C));
        self.set_reg_value(Register::A, result);
        self.update_flags(result, Some(carry), None);
    }

    fn reg_pair_add(&mut self, code1: Register, code2: Register, val: u16, set_carry: bool) {
        let old_val = self.get_reg_pair_value(code1, code2);

        let (result, carry) = math::add_16(old_val, val);

        self.set_reg_pair_value(code1, code2, result);

        if set_carry {
            self.flags.set(Flag::C, carry);
        }
    }

    fn push(&mut self, val1: u8, val2: u8) {
        self.memory.set(self.sp - 1, val1);
        self.memory.set(self.sp - 2, val2);
        self.sp -= 2;
    }

    fn pop(&mut self) -> (u8, u8) {
        let val1 = self.memory.get(self.sp + 1);
        let val2 = self.memory.get(self.sp);
        self.sp += 2;

        (val1, val2)
    }

    fn read_in_port(&mut self, port_num: u8) {
        let port = &self.in_ports[port_num as usize];
        let val = port.read();
        self.a = val;
    }

    pub fn get_in_port(&mut self, port: usize) -> u8 {
        self.in_ports[port].read()
    }

    pub fn set_in_port(&mut self, port: usize, val: u8) {
        self.in_ports[port].write(val);
    }

    pub fn set_in_port_bit(&mut self, port: usize, bit: u8, val: bool) {
        let mut port_val = self.in_ports[port].read();
        let bit_selector = 0x01u8 << bit;

        if val {
            port_val |= bit_selector;
        } else {
            port_val &= !bit_selector;
        }

        self.in_ports[port].write(port_val);
    }

    fn write_out_port(&mut self, port: u8) {
        let port = port as usize;

        self.out_ports[port].write(self.a);
    }

    pub fn get_out_port(&mut self, port: usize) -> (u8, bool) {
        let port = &mut self.out_ports[port];
        let is_dirty = port.is_dirty();
        let val = port.read();
        (val, is_dirty)
    }

    pub fn set_out_port(&mut self, port: usize, val: u8) {
        self.out_ports[port].write(val);
    }

    pub fn interrupt(&mut self, handler_num: u8) -> u8 {
        self.enable_interrupts = false;

        let addr_high = math::higher_8(self.pc);
        let addr_low = math::lower_8(self.pc);
        self.push(addr_high, addr_low);
        self.pc = ((handler_num & 0x07) << 3) as u16;

        11
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

#[cfg(test)]
mod test;
