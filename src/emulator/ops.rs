use emulator::*;
use opcode_decoder::*;

impl CPU {
    pub fn execute_op(&mut self, op: &Op, len: usize) {
        let opcode = match op.optype.upgrade() {
            Some(optype) => optype.opcode,
            None => 0x00,
        };

        let mut should_move_pc = true;

        match opcode {
            0x00 => (),
            0x01 => (),
            0x02 => (),
            0x03 | 0x13 | 0x23 => {
                let (reg1, reg2) = Register::pair_by_code(opcode >> 4);
                self.reg_pair_add(reg1, reg2, 1, false);
            }
            0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c | 0x34 | 0x3c => {
                let reg = Register::by_code(opcode >> 3);
                self.reg_add(reg, 1, false);
            }
            0x05 | 0x0d | 0x15 | 0x1d | 0x25 | 0x2d | 0x35 | 0x3d => {
                let reg = Register::by_code(opcode >> 3);
                self.reg_sub(reg, 1, false);
            }
            0x06 => (),
            0x07 => (),
            0x08 => (),
            0x09 | 0x19 | 0x29 => {
                let (reg1, reg2) = Register::pair_by_code(opcode >> 4);
                let val = self.get_reg_pair_value(reg1, reg2);
                self.reg_pair_add(Register::H, Register::L, val, true);
            }
            0x0a => (),
            0x0b | 0x1b | 0x2b => {
                let (reg1, reg2) = Register::pair_by_code(opcode >> 4);
                self.reg_pair_add(reg1, reg2, -1i16 as u16, false);
            }
            0x0e => (),
            0x0f => (),
            0x10 => (),
            0x11 => (),
            0x12 => (),
            0x16 => (),
            0x17 => (),
            0x18 => (),
            0x1a => (),
            0x1e => (),
            0x1f => (),
            0x20 => (),
            0x21 => (),
            0x22 => (),
            0x26 => (),
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
            0x28 => (),
            0x2a => (),
            0x2e => (),
            0x2f => self.a = !self.a,
            0x30 => (),
            0x31 => (),
            0x32 => (),
            0x33 => (),
            0x36 => (),
            0x37 => self.flags.set(Flag::C, true),
            0x38 => (),
            0x39 => (),
            0x3a => (),
            0x3b => (),
            0x3e => (),
            0x3f => self.flags.flip(Flag::C),
            0x76 => (),
            0x40...0x7f => {
                let src = Register::by_code(opcode);
                let dst = Register::by_code(opcode >> 3);
                self.reg_mov(dst, src);
            }
            0x80...0x87 => {
                let val = self.get_reg_value(Register::by_code(opcode));
                self.reg_add(Register::A, val, false);
            }
            0x88...0x8f => {
                let val = self.get_reg_value(Register::by_code(opcode));
                self.reg_add(Register::A, val, true);
            }
            0x90...0x97 => {
                let reg = Register::by_code(opcode);
                let val = self.get_reg_value(reg);
                self.reg_sub(Register::A, val, false);
            }
            0x98...0x9f => {
                let val = self.get_reg_value(Register::by_code(opcode));
                self.reg_sub(Register::A, val, true);
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
            0xc0 => (),
            0xc1 => (),
            0xc2 => (),
            0xc3 => (),
            0xc4 => (),
            0xc5 => (),
            0xc6 => (),
            0xc7 => (),
            0xc8 => (),
            0xc9 => (),
            0xca => (),
            0xcb => (),
            0xcc => (),
            0xcd => (),
            0xce => (),
            0xcf => (),
            0xd0 => (),
            0xd1 => (),
            0xd2 => (),
            0xd3 => (),
            0xd4 => (),
            0xd5 => (),
            0xd6 => (),
            0xd7 => (),
            0xd8 => (),
            0xd9 => (),
            0xda => (),
            0xdb => (),
            0xdc => (),
            0xdd => (),
            0xde => (),
            0xdf => (),
            0xe0 => (),
            0xe1 => (),
            0xe2 => (),
            0xe3 => (),
            0xe4 => (),
            0xe5 => (),
            0xe6 => (),
            0xe7 => (),
            0xe8 => (),
            0xe9 => (),
            0xea => (),
            0xeb => (),
            0xec => (),
            0xed => (),
            0xee => (),
            0xef => (),
            0xf0 => (),
            0xf1 => (),
            0xf2 => (),
            0xf3 => (),
            0xf4 => (),
            0xf5 => (),
            0xf6 => (),
            0xf7 => (),
            0xf8 => (),
            0xf9 => (),
            0xfa => (),
            0xfb => (),
            0xfc => (),
            0xfd => (),
            0xfe => (),
            0xff => (),
            _ => panic!("Encountered opcode outside of u16 scope"),
        }

        if should_move_pc {
            self.pc += len as u16;
        }
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
}
