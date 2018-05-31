use super::*;
use opcode_decoder::*;

fn not_implemented() {
    panic!("Not implemented");
}

pub fn get_jmp_addr(op: &Op) -> u16 {
    math::combine_8_to_16(op.arg1(), op.arg2())
}

impl CPU {
    pub fn should_jmp(&self, opcode: u8) -> bool {
        let (flag, value) = Flag::by_jmp_code(opcode);
        self.flags.is_set(flag) == value
    }

    pub fn execute_op(&mut self, op: &Op) -> u8 {
        let optype = op.optype.upgrade().unwrap();
        let opcode = optype.opcode;

        // match op {
        //     Op {
        //         arg1: Some(a1),
        //         arg2: Some(a2),
        //         ..
        //     } => println!("{} {:#04x?} {:#04x?}", op.instruction(), a1, a2),
        //     Op { arg1: Some(a1), .. } => println!("{} {:#04x?}", op.instruction(), a1),
        //     _ => println!("{}", op.instruction()),
        // }

        let mut pc_after = self.pc + optype.len as u16;
        let mut cycles = optype.cycles.0;

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
            0x32 => self
                .memory
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
            0xc0 | 0xc8 | 0xd0 | 0xd8 | 0xe0 | 0xe8 | 0xf0 | 0xf8 => if self.should_jmp(opcode) {
                let (addr1, addr2) = self.pop();
                let addr = math::combine_8_to_16(addr1, addr2);
                pc_after = addr;
                cycles = optype.cycles.1;
            },
            0xc1 | 0xd1 | 0xe1 | 0xf1 => {
                let (reg1, reg2) = Register::pair_by_code_pushpop(opcode >> 4);
                let (val1, val2) = self.pop();
                self.set_reg_value(reg1, val1);
                self.set_reg_value(reg2, val2);
            }
            0xc2 | 0xca | 0xd2 | 0xda | 0xe2 | 0xea | 0xf2 | 0xfa => if self.should_jmp(opcode) {
                pc_after = get_jmp_addr(op);
                cycles = optype.cycles.1;
            },
            0xc3 => pc_after = get_jmp_addr(op),
            0xc4 | 0xcc | 0xd4 | 0xdc | 0xe4 | 0xec | 0xf4 | 0xfc => if self.should_jmp(opcode) {
                self.push(math::higher_8(pc_after), math::lower_8(pc_after));
                pc_after = get_jmp_addr(op);
                cycles = optype.cycles.1;
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
            0xc9 => {
                let (addr1, addr2) = self.pop();
                let addr = math::combine_8_to_16(addr1, addr2);
                pc_after = addr;
            }
            0xcd => {
                self.push(math::higher_8(pc_after), math::lower_8(pc_after));
                pc_after = math::combine_8_to_16(op.arg1(), op.arg2());
            }
            0xce => self.reg_add(Register::A, op.arg1(), true, true),
            0xd3 => self.write_out_port(op.arg1()),
            0xd6 => self.reg_sub(Register::A, op.arg1(), true, false),
            0xdb => self.read_in_port(op.arg1()),
            0xde => self.reg_sub(Register::A, op.arg1(), true, true),
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
            0xe6 => self.reg_and(Register::A, op.arg1()),
            0xe9 => pc_after = math::combine_8_to_16(self.h, self.l),
            0xeb => {
                let v1 = self.get_reg_pair_value(Register::H, Register::L);
                let v2 = self.get_reg_pair_value(Register::D, Register::E);
                self.set_reg_pair_value(Register::H, Register::L, v2);
                self.set_reg_pair_value(Register::D, Register::E, v1);
            }
            0xee => self.reg_xor(Register::A, op.arg1()),
            0xf3 => self.enable_interrupts = false,
            0xf6 => self.reg_or(Register::A, op.arg1()),
            0xf9 => self.sp = self.get_reg_pair_value(Register::H, Register::L),
            0xfb => self.enable_interrupts = true,
            0xfe => self.reg_cmp(Register::A, op.arg1()),
            _ => panic!("Encountered opcode outside of u16 scope"),
        };

        self.pc = pc_after;

        cycles
    }
}
