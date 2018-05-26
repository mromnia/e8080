use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::u8;
use std::rc::Rc;
use std::rc::Weak;

pub struct Disassembler {
    opcode_register: HashMap<u8, Rc<OpType>>,
}

impl Disassembler {
    pub fn new(opcode_file: &str) -> Disassembler {
        let mut file = File::open(opcode_file).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut register = HashMap::new();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split("\t").collect();

            let mut instruction = parts[1].to_string();

            if instruction == "-" {
                continue;
            }

            let opcode = u8::from_str_radix(&parts[0][2..], 16).unwrap();

            let len = parts[2].parse::<u8>().unwrap();

            let op = OpType { opcode, instruction, len: len as usize };

            register.insert(opcode, Rc::new(op));
        }

        Disassembler { opcode_register: register }
    }

    pub fn disassemble(&self, code: &Vec<u8>) -> Vec<Op> {
        let mut pointer: usize = 0;
        let mut ops = Vec::new();

        while pointer < code.len() - 1 {
            if let Some(optype) = self.opcode_register.get(&code[pointer]) {
                let mut op = Op {
                    optype: Rc::downgrade(&optype),
                    arg1: None,
                    arg2: None
                };

                if optype.len > 1 {
                    op.arg1 = Some(code[pointer + 1]);
                }

                if optype.len > 2 {
                    op.arg2 = Some(code[pointer + 2]);
                }

                ops.push(op);

                pointer += optype.len;
            } else {
                pointer += 1;
                continue;
            }
        }

        ops
    }
}

#[derive(Debug)]
pub struct OpType {
    pub opcode: u8,
    pub instruction: String,
    pub len: usize,
}

#[derive(Debug)]
pub struct Op {
    pub optype: Weak<OpType>,
    pub arg1: Option<u8>,
    pub arg2: Option<u8>,
}
