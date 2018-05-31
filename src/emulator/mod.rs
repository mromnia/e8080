pub mod cpu;
pub mod math;

use self::cpu::*;
use opcode_decoder::*;

const CPU_HZ: i32 = 2000000;

const SHIFTED_VALUE_PORT: usize = 3;
const TO_SHIFT_VALUE_PORT: usize = 4;
const SHIFT_BITS_PORT: usize = 2;

pub struct ArcadeMachine {
    cpu: CPU,

    shift_register: u16,
}

impl ArcadeMachine {
    pub fn new(decoder: OpcodeDecoder, rom: &[u8]) -> ArcadeMachine {
        let mut cpu = CPU::new(decoder);
        cpu.set_memory(0, &rom);

        cpu.write_in_port(0, 0b00001111);
        cpu.write_in_port(1, 0b00001101);
        cpu.write_in_port(2, 0b00001000);

        ArcadeMachine {
            cpu,
            shift_register: 0,
        }
    }

    pub fn run(&mut self, t: f64) {
        let mut cycles = (CPU_HZ as f64) * t;

        while cycles > 0f64 {
            let cycles_spent = self.cpu.tick();
            cycles -= cycles_spent as f64;

            self.update_ports();
        }
    }

    fn update_ports(&mut self) {
        let mut should_update_shift = false;

        match self.cpu.read_out_port(TO_SHIFT_VALUE_PORT) {
            (v, true) => {
                self.shift_register = self.shift_register >> 8;
                let val = (v as u16) << 8;
                self.shift_register = val & self.shift_register;

                should_update_shift = true;
            }
            (_, false) => (),
        };

        match self.cpu.read_out_port(SHIFT_BITS_PORT) {
            (_, true) => should_update_shift = true,
            (_, false) => (),
        };

        if should_update_shift {
            self.update_shift_data();
        }
    }

    fn update_shift_data(&mut self) {
        let (shift, _) = self.cpu.read_out_port(SHIFT_BITS_PORT);

        let val = self.shift_register << shift;
        self.cpu.write_in_port(SHIFTED_VALUE_PORT, val as u8);
    }

    pub fn get_render_buffer(&self) -> &[u8] {
        self.cpu.get_memory_to_end(0x2400)
    }

    pub fn signal_half_render(&mut self) {
        self.cpu.interrupt(1);
    }

    pub fn signal_finish_render(&mut self) {
        self.cpu.interrupt(2);
    }
}
