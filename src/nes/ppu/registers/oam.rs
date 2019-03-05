use crate::nes::ram::Ram;

pub struct Oam {
    addr: u16,
}

impl Oam {
    pub fn new() -> Self {
        Oam {
            addr: 0
        }
    }

    pub fn reset_addr(&mut self) {
        self.addr = 0;
    }

    pub fn get_addr(&self) -> u16 {
        self.addr
    }

    pub fn write_addr(&mut self, data: u8) {
        self.addr = data as u16;
    }

    pub fn write_data(&mut self, ram: &mut Ram, data: u8) {
        ram.write(self.addr, data);
        self.addr += 1;
    }

    pub fn read_data(&self, ram: &Ram) -> u8 {
        ram.read(self.addr)
    }
}
