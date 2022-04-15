use crate::{Color, LedBar, Shape, ShapeUtility};

pub struct LedBarFacade {
    led_bars: Vec<LedBar>,
    shape_utility: ShapeUtility,
    last_midi_signal: String
}

impl LedBarFacade {
    pub fn new() -> LedBarFacade {
        LedBarFacade{
            led_bars: Vec::new(),
            shape_utility: ShapeUtility{},
            last_midi_signal: String::new()
        }
    }

    pub fn set_last_midi_signal(&mut self, last_midi_signal: String) {
        self.last_midi_signal = last_midi_signal;
    }

    pub fn add_led_bar(&mut self, led_bar: LedBar) {
        self.led_bars.push(led_bar);
    }

    pub fn animate(&mut self, shape: Shape, timestamp: f64, bps: f64) {
        let brightness = self.shape_utility.get_value_for_shape(shape, timestamp, bps);
        self.set_brightness(brightness);
    }

    pub fn get_frame(&mut self) -> [u8; 512] {
        let mut result: [u8; 512] = [0; 512];
        for led_bar in self.led_bars.iter_mut() {
            let offset = led_bar.get_dmx_offset();
            let frame = led_bar.get_frame();
            // println!("{:?}", frame);
            let mut i = 0;
            for number in frame {
                result[offset as usize + i] = number;
                i = i + 1;
            }
        }

        result
    }

    pub fn set_brightness(&mut self, brightness: f64) {
        for led_bar in self.led_bars.iter_mut() {
            led_bar.set_brightness(brightness);
        }
    }

    pub fn set_color(&mut self, color: &Color, value: u8, overwrite_existing: bool) {
        for led_bar in self.led_bars.iter_mut() {
            led_bar.set_color(color, value, overwrite_existing);
        }
    }

    pub fn reset(&mut self) {
        for led_bar in self.led_bars.iter_mut() {
            led_bar.reset();
        }
    }

    pub fn sparkle(&mut self) {
        for led_bar in self.led_bars.iter_mut() {
            led_bar.sparkle();
        }
    }
}