#[derive(Debug, Copy, Clone)]
pub struct InPort {
    data: u8,
}

impl InPort {
    pub fn new(val: u8) -> InPort {
        InPort { data: val }
    }

    pub fn read(&self) -> u8 {
        self.data
    }

    pub fn write(&mut self, val: u8) {
        self.data = val;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct OutPort {
    data: u8,
    is_dirty: bool,
}

impl OutPort {
    pub fn new(val: u8) -> OutPort {
        OutPort {
            data: val,
            is_dirty: false,
        }
    }

    pub fn read(&mut self) -> u8 {
        self.is_dirty = false;
        self.data
    }

    pub fn write(&mut self, val: u8) {
        self.data = val;
        self.is_dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }
}
