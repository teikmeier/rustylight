use serde_yaml::Value;
use serde_yaml::Mapping;
use std::time::Instant;

pub struct Fader {
    fader_type: FaderType,
    channel: usize,
    value: u8,
    current_value: u8,
    movement: Option<Movement>
}

pub enum FaderType {
    Default,
    Midi,
}

pub struct Movement {
    center: Option<u8>,
    delay_percentage: Option<u8>,
    delay_ms: Option<u8>,
    duration_percentage: Option<u8>,
    duration_ms: Option<u8>,
    max: u8,
    min: u8,
    repetition: u8,
    reverse: bool,
    shape: Shape,
}

pub enum Shape {
    Saw,
    Sine,
    Square,
    Triangle,
}

impl Fader {
    pub fn get_value(&self) -> u8 {
        self.current_value
    }

    pub fn update_state(&mut self, selected_tempo: u8, start_time: Instant) {
        if let Some(movement) = &self.movement {
            self.current_value = self.current_value + 1;
        } else {
            self.current_value = self.value;
        }
    }

    pub fn get_channel(&self) -> usize {
        self.channel
    }
}

pub fn fader_from_mapping(channel: &Value, properties: &Value) -> Option<Fader> {
    if !channel.is_number() {
        return None;
    }
    let mut fader = Fader{
        fader_type: FaderType::Default,
        channel: channel.as_u64().unwrap() as usize,
        value: 0,
        current_value: 0,
        movement: None,
    };
    if let Some(props) = properties.as_mapping() {
        for (key, value) in props.iter() {
            if key.is_string() && key.eq("value") && value.is_number() {
                fader.value = value.as_u64().unwrap() as u8;
                fader.current_value = value.as_u64().unwrap() as u8;
            } else if key.is_string() && key.eq("type") && value.is_string() {
                let fader_type = value.as_str().unwrap();
                match fader_type {
                    "default" => fader.fader_type = FaderType::Default,
                    "midi" => fader.fader_type = FaderType::Midi,
                    _ => fader.fader_type = FaderType::Default,
                }
            } else if key.is_string() && key.eq("movement") && value.is_mapping() {
                fader.movement = Some(movement_from_mapping(value.as_mapping().unwrap()));
            }
        }
    }
    return Some(fader)
}

fn movement_from_mapping(movement_input: &Mapping) -> Movement {
    let mut movement = Movement {
        center: None,
        delay_percentage: None,
        delay_ms: None,
        duration_percentage: None,
        duration_ms: None,
        max: 255,
        min: 0,
        repetition: 0,
        reverse: false,
        shape: Shape::Sine,
    };
    for (key, value) in movement_input.iter() {
        if key.is_string() && key.as_str().unwrap().eq("max") && value.is_number() {
            movement.max = value.as_u64().unwrap() as u8;
        } else if key.is_string() && key.as_str().unwrap().eq("min") && value.is_number() {
            movement.min = value.as_u64().unwrap() as u8;
        } else if key.is_string() && key.as_str().unwrap().eq("repetition") && value.is_number() {
            movement.repetition = value.as_u64().unwrap() as u8;
        } else if key.is_string() && key.as_str().unwrap().eq("reverse") && value.is_bool() {
            movement.reverse = value.as_bool().unwrap();
        } else if key.is_string() && key.as_str().unwrap().eq("shape") && value.is_string() {
            let shape = value.as_str().unwrap();
            match shape {
                "saw" => movement.shape = Shape::Saw,
                "sine" => movement.shape = Shape::Sine,
                "sqare" => movement.shape = Shape::Square,
                "triangle" => movement.shape = Shape::Triangle,
                _ => movement.shape = Shape::Sine,
            }
        } else if key.is_string() && key.as_str().unwrap().eq("center") && value.is_number() {
            movement.center = Some(value.as_u64().unwrap() as u8);
        } else if key.is_string() && key.as_str().unwrap().eq("delay_percentage") && value.is_number() {
            movement.delay_percentage = Some(value.as_u64().unwrap() as u8);
        } else if key.is_string() && key.as_str().unwrap().eq("delay_ms") && value.is_number() {
            movement.delay_ms = Some(value.as_u64().unwrap() as u8);
        } else if key.is_string() && key.as_str().unwrap().eq("duration_percentage") && value.is_number() {
            movement.duration_percentage = Some(value.as_u64().unwrap() as u8);
        } else if key.is_string() && key.as_str().unwrap().eq("duration_ms") && value.is_number() {
            movement.duration_ms = Some(value.as_u64().unwrap() as u8);
        }
    }
    movement
}
