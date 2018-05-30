#[derive(Copy, Clone)]
pub enum Flag {
    S,
    Z,
    AC,
    P,
    C,
}

impl Flag {
    pub fn bit(&self) -> u8 {
        match self {
            Flag::S => 0x80,
            Flag::Z => 0x40,
            Flag::AC => 0x10,
            Flag::P => 0x04,
            Flag::C => 0x01,
        }
    }

    pub fn by_jmp_code(code: u8) -> (Flag, bool) {
        match code & 0b00111000 {
            0b00000000 => (Flag::Z, false),
            0b00001000 => (Flag::Z, true),
            0b00010000 => (Flag::C, false),
            0b00011000 => (Flag::C, true),
            0b00100000 => (Flag::P, false),
            0b00101000 => (Flag::P, true),
            0b00110000 => (Flag::S, false),
            0b00111000 => (Flag::S, true),
            _ => panic!("Invalid jump instruction code - could not get flag"),
        }
    }
}

pub struct FlagRegister {
    flags: u8,
}

impl FlagRegister {
    pub fn new() -> FlagRegister {
        let mut reg = FlagRegister { flags: 0 };
        reg.set_all(0);
        reg
    }

    pub fn is_set(&self, flag: Flag) -> bool {
        self.flags & flag.bit() > 0
    }

    pub fn set(&mut self, flag: Flag, toggle: bool) {
        if toggle {
            self.flags = self.flags | flag.bit();
        } else {
            self.flags = self.flags & !flag.bit();
        }
    }

    pub fn flip(&mut self, flag: Flag) {
        let val = self.is_set(flag);
        self.set(flag, !val);
    }

    pub fn get_all(&self) -> u8 {
        self.flags
    }

    pub fn set_all(&mut self, val: u8) {
        self.flags = (val & 0b11010111) | 0x02;
    }
}
