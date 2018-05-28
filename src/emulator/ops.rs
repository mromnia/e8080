use emulator::*;
use opcode_decoder::*;

fn not_implemented() {
    panic!("Not implemented");
}

impl CPU {
    pub fn execute_op(&mut self, op: &Op, len: usize) {
        let opcode = match op.optype.upgrade() {
            Some(optype) => optype.opcode,
            None => 0x00,
        };

        match op {
            Op {
                arg1: Some(a1),
                arg2: Some(a2),
                ..
            } => println!("{} {:#04x?} {:#04x?}", op.instruction(), a1, a2),
            Op { arg1: Some(a1), .. } => println!("{} {:#04x?}", op.instruction(), a1),
            _ => println!("{}", op.instruction()),
        }

        let mut pc_after = self.pc + len as u16;

        match opcode {
            0x00 => (),
            0x01 | 0x11 | 0x21 | 0x31 => {
                let (reg1, reg2) = Register::pair_by_code((opcode >> 4) & 0x03);
                self.set_reg_value(reg1, op.arg1());
                self.set_reg_value(reg2, op.arg2());
            }
            0x02 | 0x12 => {
                let (reg1, reg2) = Register::pair_by_code((opcode >> 4) & 0x01);
                let addr = self.get_reg_pair_value(reg1, reg2);
                self.memory.set(addr, self.a);
            }
            0x03 | 0x13 | 0x23 | 0x33 => {
                let (reg1, reg2) = Register::pair_by_code(opcode >> 4);
                self.reg_pair_add(reg1, reg2, 1, false);
            }
            0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c | 0x34 | 0x3c => {
                let reg = Register::by_code(opcode >> 3);
                self.reg_add(reg, 1, false, false);
            }
            0x05 | 0x0d | 0x15 | 0x1d | 0x25 | 0x2d | 0x35 | 0x3d => {
                let reg = Register::by_code(opcode >> 3);
                self.reg_sub(reg, 1, false, false);
            }
            0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e | 0x36 | 0x3e => {
                let reg = Register::by_code(opcode >> 3);
                self.set_reg_value(reg, op.arg1());
            }
            0x07 | 0x17 => self.reg_rot_left((opcode & 0x10) > 0),
            0x08 | 0x10 | 0x18 | 0x28 | 0x38 | 0xcb | 0xd9 | 0xdd | 0xed | 0xfd => {
                panic!("Invalid intruction")
            }
            0x09 | 0x19 | 0x29 | 0x39 => {
                let (reg1, reg2) = Register::pair_by_code(opcode >> 4);
                let val = self.get_reg_pair_value(reg1, reg2);
                self.reg_pair_add(Register::H, Register::L, val, true);
            }
            0x0a | 0x1a => {
                let (reg1, reg2) = Register::pair_by_code((opcode >> 4) & 0x01);
                let addr = self.get_reg_pair_value(reg1, reg2);
                let val = self.memory.get(addr);
                self.set_reg_value(Register::A, val);
            }
            0x0b | 0x1b | 0x2b | 0x3b => {
                let (reg1, reg2) = Register::pair_by_code(opcode >> 4);
                self.reg_pair_add(reg1, reg2, -1i16 as u16, false);
            }
            0x0f | 0x1f => self.reg_rot_right((opcode & 0x10) > 0),
            0x20 => not_implemented(),
            0x22 => {
                let addr = math::combine_8_to_16(op.arg1(), op.arg2());
                self.memory.set(addr, self.l);
                self.memory.set(addr + 1, self.h);
            }
            0x27 => {
                if (self.a & 0x0F) > 9 || self.flags.is_set(Flag::AC) {
                    let (result, _, acarry) = math::add_8(self.a, 6);
                    self.a = result;
                    self.flags.set(Flag::AC, acarry);
                }

                if (self.a >> 4) > 9 || self.flags.is_set(Flag::C) {
                    let (result, carry, _) = math::add_8(self.a, 6 << 4);
                    self.a = result;
                    self.flags.set(Flag::C, carry);
                }
            }
            0x2a => {
                let addr = math::combine_8_to_16(op.arg1(), op.arg2());
                self.l = self.memory.get(addr);
                self.h = self.memory.get(addr + 1);
            }
            0x2f => self.a = !self.a,
            0x30 => not_implemented(),
            0x32 => self.memory
                .set(math::combine_8_to_16(op.arg1(), op.arg2()), self.a),
            0x37 => self.flags.set(Flag::C, true),
            0x3a => self.a = self.memory.get(math::combine_8_to_16(op.arg1(), op.arg2())),
            0x3f => self.flags.flip(Flag::C),
            0x76 => not_implemented(),
            0x40...0x7f => {
                let src = Register::by_code(opcode);
                let dst = Register::by_code(opcode >> 3);
                self.reg_mov(dst, src);
            }
            0x80...0x87 => {
                let val = self.get_reg_value(Register::by_code(opcode));
                self.reg_add(Register::A, val, true, false);
            }
            0x88...0x8f => {
                let val = self.get_reg_value(Register::by_code(opcode));
                self.reg_add(Register::A, val, true, true);
            }
            0x90...0x97 => {
                let reg = Register::by_code(opcode);
                let val = self.get_reg_value(reg);
                self.reg_sub(Register::A, val, true, false);
            }
            0x98...0x9f => {
                let val = self.get_reg_value(Register::by_code(opcode));
                self.reg_sub(Register::A, val, true, true);
            }
            0xa0...0xa7 => {
                let val = self.get_reg_value(Register::by_code(opcode));
                self.reg_and(Register::A, val);
            }
            0xa8...0xaf => {
                let val = self.get_reg_value(Register::by_code(opcode));
                self.reg_xor(Register::A, val);
            }
            0xb0...0xb7 => {
                let val = self.get_reg_value(Register::by_code(opcode));
                self.reg_or(Register::A, val);
            }
            0xb8...0xbf => {
                let val = self.get_reg_value(Register::by_code(opcode));
                self.reg_cmp(Register::A, val);
            }
            0xc0 => if !self.flags.is_set(Flag::Z) {
                let (addr1, addr2) = self.pop();
                let addr = math::combine_8_to_16(addr1, addr2);
                pc_after = addr;
            },
            0xc1 | 0xd1 | 0xe1 | 0xf1 => {
                let (reg1, reg2) = Register::pair_by_code_pushpop(opcode >> 4);
                let (val1, val2) = self.pop();
                self.set_reg_value(reg1, val1);
                self.set_reg_value(reg2, val2);
            }
            0xc2 => if !self.flags.is_set(Flag::Z) {
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xc3 => pc_after = math::combine_8_to_16(op.arg1(), op.arg2()),
            0xc4 => if !self.flags.is_set(Flag::Z) {
                self.push(math::higher_8(pc_after), math::lower_8(pc_after));
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xc5 | 0xd5 | 0xe5 | 0xf5 => {
                let (reg1, reg2) = Register::pair_by_code_pushpop(opcode >> 4);
                let val1 = self.get_reg_value(reg1);
                let val2 = self.get_reg_value(reg2);
                self.push(val1, val2);
            }
            0xc6 => self.reg_add(Register::A, op.arg1(), true, false),
            0xc7 | 0xcf | 0xd7 | 0xdf | 0xe7 | 0xef | 0xf7 | 0xff => {
                self.push(math::higher_8(pc_after), math::lower_8(pc_after));
                let exp = op.arg1() & 0x38;
                pc_after = exp as u16;
            }
            0xc8 => if self.flags.is_set(Flag::Z) {
                let (addr1, addr2) = self.pop();
                let addr = math::combine_8_to_16(addr1, addr2);
                pc_after = addr;
            },
            0xc9 => {
                let (addr1, addr2) = self.pop();
                let addr = math::combine_8_to_16(addr1, addr2);
                pc_after = addr;
            }
            0xca => if self.flags.is_set(Flag::Z) {
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xcc => if self.flags.is_set(Flag::Z) {
                self.push(math::higher_8(pc_after), math::lower_8(pc_after));
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xcd => {
                self.push(math::higher_8(pc_after), math::lower_8(pc_after));
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            }
            0xce => self.reg_add(Register::A, op.arg1(), true, true),
            0xd0 => if !self.flags.is_set(Flag::C) {
                let (addr1, addr2) = self.pop();
                let addr = math::combine_8_to_16(addr1, addr2);
                pc_after = addr;
            },
            0xd2 => if !self.flags.is_set(Flag::C) {
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xd3 => (),
            0xd4 => if !self.flags.is_set(Flag::C) {
                self.push(math::higher_8(pc_after), math::lower_8(pc_after));
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xd6 => self.reg_sub(Register::A, op.arg1(), true, false),
            0xd8 => if self.flags.is_set(Flag::C) {
                let (addr1, addr2) = self.pop();
                let addr = math::combine_8_to_16(addr1, addr2);
                pc_after = addr;
            },
            0xda => if self.flags.is_set(Flag::C) {
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xdb => (),
            0xdc => if self.flags.is_set(Flag::C) {
                self.push(math::higher_8(pc_after), math::lower_8(pc_after));
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xde => self.reg_sub(Register::A, op.arg1(), true, true),
            0xe0 => if !self.flags.is_set(Flag::P) {
                let (addr1, addr2) = self.pop();
                let addr = math::combine_8_to_16(addr1, addr2);
                pc_after = addr;
            },
            0xe2 => if !self.flags.is_set(Flag::P) {
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xe3 => {
                let reg_h = self.get_reg_value(Register::H);
                let reg_l = self.get_reg_value(Register::L);

                let memory_higher = self.memory.get(self.sp + 1);
                let memory_lower = self.memory.get(self.sp);

                self.set_reg_value(Register::H, memory_higher);
                self.set_reg_value(Register::L, memory_lower);

                self.memory.set(self.sp + 1, reg_h);
                self.memory.set(self.sp, reg_l);
            }
            0xe4 => if !self.flags.is_set(Flag::P) {
                self.push(math::higher_8(pc_after), math::lower_8(pc_after));
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xe6 => self.reg_and(Register::A, op.arg1()),
            0xe8 => if self.flags.is_set(Flag::P) {
                let (addr1, addr2) = self.pop();
                let addr = math::combine_8_to_16(addr1, addr2);
                pc_after = addr;
            },
            0xe9 => pc_after = math::combine_8_to_16(self.h, self.l),
            0xea => if self.flags.is_set(Flag::P) {
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xeb => {
                let v1 = self.get_reg_pair_value(Register::H, Register::L);
                let v2 = self.get_reg_pair_value(Register::D, Register::E);
                self.set_reg_pair_value(Register::H, Register::L, v2);
                self.set_reg_pair_value(Register::D, Register::E, v1);
            }
            0xec => if self.flags.is_set(Flag::P) {
                self.push(math::higher_8(pc_after), math::lower_8(pc_after));
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xee => self.reg_xor(Register::A, op.arg1()),
            0xf0 => if !self.flags.is_set(Flag::S) {
                let (addr1, addr2) = self.pop();
                let addr = math::combine_8_to_16(addr1, addr2);
                pc_after = addr;
            },
            0xf2 => if !self.flags.is_set(Flag::S) {
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xf3 => self.enable_interrupts = false,
            0xf4 => if !self.flags.is_set(Flag::S) {
                self.push(math::higher_8(pc_after), math::lower_8(pc_after));
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xf6 => self.reg_or(Register::A, op.arg1()),
            0xf8 => if self.flags.is_set(Flag::S) {
                let (addr1, addr2) = self.pop();
                let addr = math::combine_8_to_16(addr1, addr2);
                pc_after = addr;
            },
            0xf9 => self.sp = self.get_reg_pair_value(Register::H, Register::L),
            0xfa => if self.flags.is_set(Flag::S) {
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xfb => self.enable_interrupts = true,
            0xfc => if self.flags.is_set(Flag::S) {
                self.push(math::higher_8(pc_after), math::lower_8(pc_after));
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            },
            0xfe => self.reg_cmp(Register::A, op.arg1()),
            _ => panic!("Encountered opcode outside of u16 scope"),
        }

        self.pc = pc_after;
    }
}

#[cfg(test)]
mod test {
    extern crate rand;

    use emulator::ops::test::rand::prelude::*;
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
    fn test_inr_carry() {
        let mut cpu = CPU::new(init_decoder());

        set_op_at_rnd_addr(&mut cpu, 0x3c);
        cpu.a = 255;
        cpu.tick();

        assert_eq!(cpu.a as i8, 0);
        assert!(!cpu.flags.is_set(Flag::S));
        assert!(cpu.flags.is_set(Flag::Z));
        assert!(cpu.flags.is_set(Flag::C));
        assert!(cpu.flags.is_set(Flag::P));
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
    fn test_dcr_carry() {
        let mut cpu = CPU::new(init_decoder());

        set_op_at_rnd_addr(&mut cpu, 0x15);
        cpu.d = 0;
        cpu.tick();

        assert_eq!(cpu.d as i8, -1);
        assert!(cpu.flags.is_set(Flag::S));
        assert!(!cpu.flags.is_set(Flag::Z));
        assert!(cpu.flags.is_set(Flag::C));
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
}
