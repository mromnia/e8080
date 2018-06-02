pub mod cpu;
pub mod math;

use self::cpu::*;
use opcode_decoder::*;

const CPU_HZ: i32 = 2000000;

const SHIFTED_VALUE_PORT: usize = 3;
const VALUE_TO_SHIFT_PORT: usize = 4;
const SHIFT_BY_BITS_PORT: usize = 2;

pub struct ArcadeMachine {
    cpu: CPU,

    shift_register: u16,
}

impl ArcadeMachine {
    pub fn new(decoder: OpcodeDecoder, rom: &[u8]) -> ArcadeMachine {
        let mut cpu = CPU::new(decoder);
        cpu.set_memory(0, &rom);

        cpu.set_in_port(0, 0b00001110);
        cpu.set_in_port(1, 0b00001000);
        cpu.set_in_port(2, 0b00001000);

        ArcadeMachine {
            cpu,
            shift_register: 0,
        }
    }

    pub fn run(&mut self, t: f64) -> u32 {
        let cycles_to_run = (CPU_HZ as f64) * t;
        let mut cycles = cycles_to_run;

        while cycles > 0.0 {
            let cycles_spent = self.cpu.tick();
            cycles -= cycles_spent as f64;

            self.update_ports();
        }

        (cycles_to_run - cycles) as u32
    }

    fn update_ports(&mut self) {
        let mut should_update_shift = false;

        match self.cpu.get_out_port(VALUE_TO_SHIFT_PORT) {
            (v, true) => {
                self.update_shift_register(v);
                should_update_shift = true;
            }
            (_, false) => (),
        };

        match self.cpu.get_out_port(SHIFT_BY_BITS_PORT) {
            (_, true) => should_update_shift = true,
            (_, false) => (),
        };

        if should_update_shift {
            self.update_shift_data();
        }
    }

    fn update_shift_register(&mut self, new_val: u8) {
        self.shift_register = self.shift_register >> 8;
        let new_val = (new_val as u16) << 8;
        self.shift_register = new_val | self.shift_register;
    }

    fn update_shift_data(&mut self) {
        let (shift, _) = self.cpu.get_out_port(SHIFT_BY_BITS_PORT);
        let shift = shift & 0x07;
        let val = (self.shift_register << shift) >> 8;
        self.cpu.set_in_port(SHIFTED_VALUE_PORT, val as u8);
    }

    pub fn get_render_buffer(&self) -> &[u8] {
        println!("{:?}", self.shift_register);
        self.cpu.get_memory_to_end(0x2400)
    }

    pub fn signal_half_render(&mut self) {
        self.cpu.interrupt(1);
    }

    pub fn signal_finish_render(&mut self) {
        self.cpu.interrupt(2);
    }

    pub fn coin_key_toggle(&mut self, down: bool) {
        self.cpu.set_in_port_bit(1, 0, down);
    }

    pub fn start_p1_key_toggle(&mut self, down: bool) {
        self.cpu.set_in_port_bit(1, 2, down);
    }

    pub fn fire_p1_key_toggle(&mut self, down: bool) {
        self.cpu.set_in_port_bit(1, 4, down);
    }

    pub fn left_p1_key_toggle(&mut self, down: bool) {
        self.cpu.set_in_port_bit(1, 5, down);
    }

    pub fn right_p1_key_toggle(&mut self, down: bool) {
        self.cpu.set_in_port_bit(1, 6, down);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    fn init_decoder() -> OpcodeDecoder {
        let mut opcode_data = String::new();
        {
            let mut opcode_file = File::open("./data/opcodes.txt").unwrap();
            opcode_file.read_to_string(&mut opcode_data).unwrap();
        }
        OpcodeDecoder::new(&opcode_data)
    }

    #[test]
    fn test_shift_register() {
        let mut machine = ArcadeMachine::new(init_decoder(), &[]);

        machine.cpu.set_out_port(VALUE_TO_SHIFT_PORT, 0b01110011);
        machine.update_ports();

        assert_eq!(machine.cpu.get_in_port(SHIFTED_VALUE_PORT), 0b01110011);
    }

    #[test]
    fn test_shift_register_offset() {
        let mut machine = ArcadeMachine::new(init_decoder(), &[]);

        machine.cpu.set_out_port(VALUE_TO_SHIFT_PORT, 0b01110011);
        machine.cpu.set_out_port(SHIFT_BY_BITS_PORT, 3);
        machine.update_ports();

        assert_eq!(machine.cpu.get_in_port(SHIFTED_VALUE_PORT), 0b10011000);
    }

    #[test]
    fn test_shift_register_multiple() {
        let mut machine = ArcadeMachine::new(init_decoder(), &[]);

        machine.cpu.set_out_port(VALUE_TO_SHIFT_PORT, 0b01110011);
        machine.update_ports();

        assert_eq!(machine.cpu.get_in_port(SHIFTED_VALUE_PORT), 0b01110011);

        machine.cpu.set_out_port(VALUE_TO_SHIFT_PORT, 0b01010101);

        machine.cpu.set_out_port(SHIFT_BY_BITS_PORT, 3);
        machine.cpu.get_out_port(SHIFT_BY_BITS_PORT);

        machine.update_ports();

        assert_eq!(machine.cpu.get_in_port(SHIFTED_VALUE_PORT), 0b10101011);
    }

    #[test]
    fn test_shift_register_multiple2() {
        let mut machine = ArcadeMachine::new(init_decoder(), &[]);

        machine.cpu.set_out_port(VALUE_TO_SHIFT_PORT, 0x38);
        machine.update_ports();
        machine.cpu.set_out_port(VALUE_TO_SHIFT_PORT, 0xF1);
        machine.update_ports();
        machine.cpu.set_out_port(VALUE_TO_SHIFT_PORT, 0xFF);
        machine.update_ports();
        machine.cpu.set_out_port(VALUE_TO_SHIFT_PORT, 0x80);
        machine.update_ports();
        machine.cpu.set_out_port(VALUE_TO_SHIFT_PORT, 0x0E);
        machine.cpu.set_out_port(SHIFT_BY_BITS_PORT, 3);
        machine.update_ports();

        assert_eq!(machine.cpu.get_in_port(SHIFTED_VALUE_PORT), 0b01110100);
    }
}
