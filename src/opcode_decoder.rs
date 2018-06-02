use std::collections::HashMap;
use std::rc::Rc;
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

            let cycles = if parts[3].contains("/") {
                let cycle_parts: Vec<u8> = parts[3]
                    .split("/")
                    .map(|p| p.parse::<u8>().unwrap())
                    .collect();
                (cycle_parts[0], cycle_parts[1])
            } else {
                let cycles = parts[3].parse::<u8>().unwrap();
                (cycles, cycles)
            };

            let op = OpType {
                opcode,
                instruction,
                len: len as usize,
                cycles,
            };

            register.insert(opcode, Rc::new(op));
        }

        OpcodeDecoder { opcodes: register }
    }

    pub fn get_next_op(&self, program: &[u8]) -> Result<Op, String> {
        self.print_10_ops();

        if let Some(optype) = self.opcodes.get(&program[0]) {
            let mut op = Op {
                optype: Rc::clone(&optype),
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

            Ok(op)
        } else {
            Err(format!("Invalid opcode: {:#04x?}", program[0]))
        }
    }

    pub fn print_10_ops(&self) {
        let mut counter = 0;
        for (opcode, _) in &self.opcodes {
            println!("{}", opcode);
            counter += 1;

            if counter >= 10 {
                println!("{}", "--------------");
                return;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct OpType {
    pub opcode: u8,
    pub instruction: String,
    pub len: usize,
    pub cycles: (u8, u8),
}

#[derive(Debug, Clone)]
pub struct Op {
    pub optype: Rc<OpType>,
    pub arg1: Option<u8>,
    pub arg2: Option<u8>,
}

impl Op {
    pub fn instruction(&self) -> String {
        self.optype.instruction.to_string()
    }

    pub fn arg1(&self) -> u8 {
        match self.arg1 {
            Some(a) => a,
            None => panic!("Expected arg1 in Op"),
        }
    }

    pub fn arg2(&self) -> u8 {
        match self.arg2 {
            Some(a) => a,
            None => panic!("Expected arg2 in Op"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Op {
                arg1: Some(a1),
                arg2: Some(a2),
                ..
            } => format!("{} {:#04x?} {:#04x?}", self.instruction(), a1, a2),
            Op { arg1: Some(a1), .. } => format!("{} {:#04x?}", self.instruction(), a1),
            _ => format!("{}", self.instruction()),
        }
    }
}
