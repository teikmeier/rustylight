use rand::Rng;
use crate::ShapeUtility;

pub enum Color {
    Red,
    Green,
    Blue,
    White,
    Amber,
    Black
}

pub struct Led {
    brightness: f64,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub white: u8,
    pub amber: u8,
    pub black: u8,
    pub offset: u8
}

impl Led {
    pub fn new(offset: u8) -> Led {
        Led {
            brightness: 1.0,
            red: 0,
            green: 0,
            blue: 0,
            white: 0,
            amber: 0,
            black: 0,
            offset
        }
    }

    pub fn set_color(&mut self, color: &Color, value: u8, overwrite_existing: bool) {
        if overwrite_existing {
            self.reset();
        }

        match color {
            Color::Red => self.red = value,
            Color::Green => self.green = value,
            Color::Blue => self.blue = value,
            Color::White => self.white = value,
            Color::Amber => self.amber = value,
            Color::Black => self.black = value,
        }
    }

    pub fn set_brightness(&mut self, value: f64) {
        if value <= 1.0 && (value > 0.0 || value == 0.0) {
            self.brightness = value;
        }
    }

    pub fn reset(&mut self) {
        self.brightness = 1.0;
        self.red = 0;
        self.green = 0;
        self.blue = 0;
        self.white = 0;
        self.amber = 0;
        self.black = 0;
    }
}

// 0 - helligkeit gesamt
// 1 - strobo gesamt

// 2 - rot
// 3 - grün
// 4 - blau
// 5 - weiß
// 6 - amber
// 7 - schwarz

pub struct LedBar {
    shape_utility: ShapeUtility,
    global_brightness: u8,
    global_strobe: u8,
    leds: Vec<Led>,
    dmx_offset: u8
}

impl LedBar {
    pub fn new(dmx_offset: u8) -> LedBar {
        LedBar {
            shape_utility: ShapeUtility{},
            global_brightness: 0,
            global_strobe: 0,
            leds: Vec::new(),
            dmx_offset,
        }
    }

    pub fn add_led(&mut self, led: Led) {
        self.leds.push(led);
    }

    pub fn set_global_brightness(&mut self, value: u8) {
        self.global_brightness = value;
    }

    pub fn set_global_strobe(&mut self, value: u8) {
        self.global_strobe = value;
    }

    pub fn get_dmx_offset(&mut self) -> u8 {
        self.dmx_offset
    }

    pub fn set_color(&mut self, color: &Color, value: u8, overwrite_existing: bool) {
        for led in self.leds.iter_mut() {
            led.set_color(&color, value, overwrite_existing);
        }
    }

    pub fn set_brightness(&mut self, value: f64) {
        for led in self.leds.iter_mut() {
            if value > 1.0 {
                led.set_brightness(1.0);
            } else if value < 0.0 {
                led.set_brightness(0.0);
            } else {
                led.set_brightness(value);
            }
        }
    }

    pub fn sparkle(&mut self) {
        for led in self.leds.iter_mut() {
            let middle_value = (rand::thread_rng().gen_range(0..50) as f64) / 100.0;

        }
    }

    pub fn reset(&mut self) {
        for led in self.leds.iter_mut() {
            led.reset();
        }
    }

    pub fn get_frame(&mut self) -> Vec<u8> {
        let mut frame = Vec::new();
        frame.push(self.global_brightness);
        frame.push(self.global_strobe);


        let mut i: usize = 2;
        for led in &self.leds {
            frame.push((led.red as f64 * led.brightness).round() as u8);
            frame.push((led.green as f64 * led.brightness).round() as u8);
            frame.push((led.blue as f64 * led.brightness).round() as u8);
            frame.push((led.white as f64 * led.brightness).round() as u8);
            frame.push((led.amber as f64 * led.brightness).round() as u8);
            frame.push((led.black as f64 * led.brightness).round() as u8);
            i = i + 6;
        }

        frame
    }
}
