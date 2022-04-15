use std::fmt::{Debug, Formatter, Result};

pub struct DmxBuffer {
    values: [u8; 512]
}

impl DmxBuffer {
    pub fn new() -> DmxBuffer {
        DmxBuffer {
            values: [0; 512]
        }
    }

    // Set a single DMX value to a channel
    pub fn set_value(&mut self, dmx_channel: usize, value: u8) {
        self.values[dmx_channel] = value;
    }

    pub fn set_values(&mut self, values: [u8; 512]) {
        self.values = values;
    }

    pub fn get_values(&self) -> [u8; 512] {
        self.values
    }

    // No checks for out of bounds happen here
    pub fn write_at_index(&mut self, offset: u8, values: Vec<u8>) {
        let mut i = 0;
        for value in values {
            self.values[offset as usize + i] = value;
            i = i + 1;
        }
    }

    pub fn reset(&mut self) {
        self.values = [0; 512];
    }
}

impl Debug for DmxBuffer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.values)
    }
}