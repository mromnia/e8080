use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;
use std::u8;

pub struct OpcodeDecoder {
    opcodes: HashMap<u8, Rc<OpType>>,
}

impl OpcodeDecoder {
    pub fn new(opcode_data: &str) -> OpcodeDecoder {
        let mut register = HashMap::new();

        for line in opcode_data.lines() {
            let parts: Vec<&str> = line.split("\t").collect();

            let mut instruction = parts[1].to_string();

            if instruction == "-" {
                continue;
            }

            let opcode = u8::from_str_radix(&parts[0][2..], 16).unwrap();

            let len = parts[2].parse::<u8>().unwrap();

            let op = OpType {
                opcode,
                instruction,
                len: len as usize,
            };

            register.insert(opcode, Rc::new(op));
        }

        OpcodeDecoder { opcodes: register }
    }

    pub fn get_next_op(&self, program: &[u8]) -> Result<(Op, usize), &'static str> {
        if let Some(optype) = self.opcodes.get(&program[0]) {
            let mut op = Op {
                optype: Rc::downgrade(&optype),
                arg1: None,
                arg2: None,
            };

            if optype.len > 1 {
                op.arg1 = Some(program[1]);
            }

            if optype.len > 2 {
                op.arg2 = op.arg1;
                op.arg1 = Some(program[2]);
            }

            Ok((op, optype.len))
        } else {
            Err("Invalid opcode")
        }
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

impl Op {
    pub fn instruction(&self) -> String {
        if let Some(optype) = self.optype.upgrade() {
            optype.instruction.to_string()
        } else {
            panic!("Attempt to get missing OpType")
        }
    }
}
