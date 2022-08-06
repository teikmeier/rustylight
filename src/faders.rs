use crate::configuration::BaseConfig;
use serde_yaml::Value;
use serde_yaml::Mapping;
use std::time::Instant;
use std::f64::consts::PI;
use std::fmt;
use log::{debug, trace};

pub struct Fader {
    fader_type: FaderType,
    channel: usize,
    value: u8,
    current_value: u8,
    movement: Option<Movement>,
    midi_params: Option<MidiParams>,
    timeout_start: Option<Instant>,
}

#[derive(Debug)]
pub enum FaderType {
    Default,
    Midi,
}

impl fmt::Display for FaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Movement {
    delay_percentage: Option<u64>,
    delay_ms: Option<u64>,
    duration_percentage: Option<u64>,
    duration_ms: Option<u64>,
    max: u8,
    min: u8,
    curve_max: Option<i64>,
    curve_min: Option<i64>,
    repetition: u8,
    reverse: bool,
    shape: Shape,
}

#[derive(Debug)]
pub enum Shape {
    Saw,
    Sine,
    Square,
    Triangle,
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct MidiParams {
    note: u8,
    timeout: u64,
}

impl Fader {
    pub fn get_value(&self) -> u8 {
        match &self.fader_type {
            FaderType::Default => self.current_value,
            FaderType::Midi => {
                if self.current_value != 0 {
                    debug!("Midi fader set to {}", self.current_value);
                }
                0
            },
        }
    }

    pub fn update_state(&mut self, selected_tempo: u8, start_time: Instant, notes: [Option<u8>; 128], config: &BaseConfig) {
        match &self.fader_type {
            FaderType::Default => {
                if let Some(movement) = &self.movement {
                    self.current_value = calculate_movement(movement, selected_tempo, start_time);
                } else {
                    self.current_value = self.value;
                }
            },
            FaderType::Midi => {
                if let Some(midi_params) = &self.midi_params {
                    if let Some(note_velocity) = notes[midi_params.note as usize] {
                        if note_velocity > 0 {
                            self.timeout_start = Some(Instant::now());
                            self.current_value = self.value;
                        } else {
                            trace!("Stopped due to note off");
                            self.current_value = 0;
                        }
                    }
                    if let Some(timeout_start) = &self.timeout_start {
                        if timeout_start.elapsed().as_millis() as u64 >= midi_params.timeout {
                            trace!("Reached time out after {}ms", midi_params.timeout);
                            self.current_value = 0;
                            self.timeout_start = None;
                        }
                    }
                    if !config.midi_faders {
                        self.current_value = 0;
                    }
                } else {
                    self.current_value = 0;
                }
            }
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
        midi_params: None,
        timeout_start: None,
    };
    if let Some(props) = properties.as_mapping() {
        for (key, value) in props.iter() {
            if key.is_string() && key.eq("value") && value.is_number() {
                fader.value = value.as_u64().unwrap() as u8;
            } else if key.is_string() && key.eq("type") && value.is_string() {
                let fader_type = value.as_str().unwrap();
                match fader_type {
                    "default" => fader.fader_type = FaderType::Default,
                    "midi" => fader.fader_type = FaderType::Midi,
                    _ => fader.fader_type = FaderType::Default,
                }
            } else if key.is_string() && key.eq("movement") && value.is_mapping() {
                fader.movement = Some(movement_from_mapping(value.as_mapping().unwrap()));
            } else if key.is_string() && key.eq("params") && value.is_mapping() {
                fader.midi_params = Some(midi_params_from_mapping(value.as_mapping().unwrap()));
            }
        }
    }
    return Some(fader)
}

fn movement_from_mapping(movement_input: &Mapping) -> Movement {
    let mut movement = Movement {
        delay_percentage: None,
        delay_ms: None,
        duration_percentage: None,
        duration_ms: None,
        max: 255,
        min: 0,
        curve_max: None,
        curve_min: None,
        repetition: 0,
        reverse: false,
        shape: Shape::Sine,
    };
    for (key, value) in movement_input.iter() {
        if key.is_string() && key.as_str().unwrap().eq("max") && value.is_number() {
            movement.max = value.as_u64().unwrap() as u8;
        } else if key.is_string() && key.as_str().unwrap().eq("min") && value.is_number() {
            movement.min = value.as_u64().unwrap() as u8;
        } else if key.is_string() && key.as_str().unwrap().eq("max_percentage") && value.is_number() {
            movement.max = ((value.as_f64().unwrap() / 100.0) * 255.0) as u8;
        } else if key.is_string() && key.as_str().unwrap().eq("min_percentage") && value.is_number() {
            movement.min = ((value.as_f64().unwrap() / 100.0) * 255.0) as u8;
        } else if key.is_string() && key.as_str().unwrap().eq("curve_max") && value.is_number() {
            movement.curve_max = value.as_i64();
        } else if key.is_string() && key.as_str().unwrap().eq("curve_min") && value.is_number() {
            movement.curve_min = value.as_i64();
        } else if key.is_string() && key.as_str().unwrap().eq("curve_max_percentage") && value.is_number() {
            movement.curve_max = Some(((value.as_f64().unwrap() / 100.0) * 255.0) as i64);
        } else if key.is_string() && key.as_str().unwrap().eq("curve_min_percentage") && value.is_number() {
            movement.curve_min = Some(((value.as_f64().unwrap() / 100.0) * 255.0) as i64);
        } else if key.is_string() && key.as_str().unwrap().eq("repetition") && value.is_number() {
            movement.repetition = value.as_u64().unwrap() as u8;
        } else if key.is_string() && key.as_str().unwrap().eq("reverse") && value.is_bool() {
            movement.reverse = value.as_bool().unwrap();
        } else if key.is_string() && key.as_str().unwrap().eq("shape") && value.is_string() {
            let shape = value.as_str().unwrap();
            match shape {
                "saw" => movement.shape = Shape::Saw,
                "sine" => movement.shape = Shape::Sine,
                "square" => movement.shape = Shape::Square,
                "triangle" => movement.shape = Shape::Triangle,
                _ => movement.shape = Shape::Sine,
            }
        } else if key.is_string() && key.as_str().unwrap().eq("delay_percentage") && value.is_number() {
            movement.delay_percentage = Some(value.as_u64().unwrap());
        } else if key.is_string() && key.as_str().unwrap().eq("delay_ms") && value.is_number() {
            movement.delay_ms = Some(value.as_u64().unwrap());
        } else if key.is_string() && key.as_str().unwrap().eq("duration_percentage") && value.is_number() {
            movement.duration_percentage = value.as_u64();
        } else if key.is_string() && key.as_str().unwrap().eq("duration_ms") && value.is_number() {
            movement.duration_ms = value.as_u64();
        }
    }
    // Ensure max is bigger than min
    if movement.max < movement.min {
        movement.min = movement.max;
    }
    // Ensure curve max and min are set
    if movement.curve_max.is_none() {
        movement.curve_max = Some(movement.max as i64);
    }
    if movement.curve_min.is_none() {
        movement.curve_min = Some(movement.min as i64);
    }
    // Ensure either percentage or ms is set
    if movement.duration_percentage.is_none() && movement.duration_ms.is_none() {
        movement.duration_percentage = Some(400);
    }
    if movement.delay_percentage.is_none() && movement.delay_ms.is_none() {
        movement.delay_percentage = Some(0);
    }
    movement
}

fn midi_params_from_mapping(midi_params_input: &Mapping) -> MidiParams {
    let mut midi_params = MidiParams {
        note: 0,
        timeout: 2000 // Default timeout of 2 seconds
    };
    for (key, value) in midi_params_input.iter() {
        if key.is_string() && key.as_str().unwrap().eq("note") && value.is_number() {
            midi_params.note = value.as_u64().unwrap() as u8;
        } else if key.is_string() && key.as_str().unwrap().eq("timeout_ms") && value.is_number() {
            midi_params.timeout = value.as_u64().unwrap();
        }
    }
    midi_params
}

fn calculate_movement(movement: &Movement, beats_per_minute: u8, start_time: Instant) -> u8 {
    let max = movement.max as f64;
    let min = movement.min as f64;
    let curve_max = movement.curve_max.unwrap() as f64;
    let curve_min = movement.curve_min.unwrap() as f64;
    let beat_duration_ms: f64 = 60000.0 / beats_per_minute as f64;
    let movement_duration_ms: f64 = if let Some(percentage) = movement.duration_percentage {
        beat_duration_ms * (percentage as f64 / 100.0)
    } else if let Some(ms) = movement.duration_ms {
        ms as f64
    } else {
        beat_duration_ms * 4.0
    };
    let delay_ms = if let Some(percentage) = movement.delay_percentage {
        movement_duration_ms * (percentage as f64 / 100.0)
    } else if let Some(ms) = movement.delay_ms {
        ms as f64
    } else {
        0.0
    };
    let elapsed_ms = start_time.elapsed().as_millis() as f64;
    let current_position = ((elapsed_ms + delay_ms) % movement_duration_ms) / movement_duration_ms;
    let new_value: f64;

    match movement.shape {
        Shape::Sine => {
            let new_unified_value = cos_function(current_position, movement.reverse);
            new_value = ((curve_max - curve_min) * new_unified_value) + curve_min;
        },
        Shape::Square => {
            if current_position < 0.5 && movement.reverse {
                new_value = curve_max;
            } else if current_position >= 0.5 && movement.reverse {
                new_value = curve_min;
            } else if current_position < 0.5 && !movement.reverse {
                new_value = curve_min;
            } else {
                new_value = curve_max;
            }
        },
        Shape::Saw => {
            let new_unified_value = saw_function(current_position, movement.reverse);
            new_value = ((curve_max - curve_min) * new_unified_value) + curve_min;
        },
        Shape::Triangle => {
            let new_unified_value = triangle_function(current_position, movement.reverse);
            new_value = ((curve_max - curve_min) * new_unified_value) + curve_min;
        }
    }

    if new_value >= max {
        max as u8
    } else if new_value <= min {
        min as u8
    } else {
        new_value as u8
    }
}

fn cos_function(mut position: f64, reverse: bool) -> f64 {
    let mut reverse_operator = -1.0;
    if position > 1.0 {
        position = 1.0;
    } else if position < 0.0 {
        position = 0.0;
    }
    if reverse {
        reverse_operator = 1.0;
    }
    (-1.0 * reverse_operator * f64::cos( 2.0 * PI * position ) + 1.0) / 2.0
}

fn saw_function(mut position: f64, reverse: bool) -> f64 {
    if position > 1.0 {
        position = 1.0;
    } else if position < 0.0 {
        position = 0.0;
    }
    if reverse {
        return 1.0 - position;
    }
    return position;
}

fn triangle_function(mut position: f64, reverse: bool) -> f64 {
    if position > 1.0 {
        position = 1.0;
    } else if position < 0.0 {
        position = 0.0;
    }
    if !reverse {
        if position <= 0.5 {
            position * 2.0
        } else {
            1.0 - ((position - 0.5) * 2.0)
        }
    } else {
        if position <= 0.5 {
            1.0 - (position * 2.0)
        } else {
            (position - 0.5) * 2.0
        }
    }
}
