#[derive(Debug, Copy, Clone)]
pub struct Port {
    data: u8,
}

impl Port {
    pub fn new(val: u8) -> Port {
        Port { data: val }
    }

    pub fn get(&self) -> u8 {
        self.data
    }

    pub fn set(&mut self, val: u8) {
        self.data = val;
    }

    pub fn read(&mut self) -> u8 {
        self.data
    }

    pub fn write(&mut self, val: u8) {
        self.data = val;
    }
}
