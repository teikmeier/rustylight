use std::f64::consts::PI;
use std::cmp::{min, max};

pub enum Shape {
    Sine,
    Square,
    Sparkle
}

pub struct ShapeUtility {
}

impl ShapeUtility {
    pub fn get_value_for_shape(&self, shape: Shape, step: f64, bps: f64) -> f64 {
        match shape {
            Shape::Sine | Shape::Sparkle => {
                self.calculate_cos_value(step, bps, 0.5)
            },
            Shape::Square => {
                let result = self.calculate_cos_value(step, bps, 0.5);
                if result > 0.5 {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }

    pub fn calculate_cos_value(&self, step: f64, bps: f64, mut amplitude: f64) -> f64 {
        if amplitude > 0.5 {
            amplitude = 0.5;
        }

        if amplitude < -0.5 {
            amplitude = -0.5;
        }

        let b = 2.0 * PI * bps;
        let c = 1.0 / (bps * 2.0);

        amplitude * (-f64::cos(b * (step - c))) + 0.5
    }
}