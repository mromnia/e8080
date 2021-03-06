use std::u8;

use opcode_decoder;

pub struct Disassembler {
    opcode_decoder: opcode_decoder::OpcodeDecoder,
}

impl Disassembler {
    pub fn new(opcode_decoder: opcode_decoder::OpcodeDecoder) -> Disassembler {
        Disassembler { opcode_decoder }
    }

    pub fn disassemble(&self, code: &[u8]) -> Vec<opcode_decoder::Op> {
        let mut pointer: usize = 0;
        let mut ops = Vec::new();

        while pointer < code.len() - 1 {
            match self.opcode_decoder.get_next_op(&code[pointer..]) {
                Ok(op) => {
                    let optype = op.optype.upgrade().unwrap();
                    ops.push(op);
                    pointer += optype.len;
                }
                Err(err) => panic!(err),
            }
        }

        ops
    }
}
