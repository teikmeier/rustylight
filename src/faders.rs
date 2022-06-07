use serde_yaml::Value;

pub enum FaderType {
    Default,
    Midi,
}

pub struct Fader {
    fader_type: FaderType,
    channel: usize,
    value: u8,
}

impl Fader {
    pub fn get_value(&self) -> u8 {
        self.value
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
        value: 0
    };
    if let Some(props) = properties.as_mapping() {
        for (key, value) in props.iter() {
            if key.is_string() && key.eq("value") && value.is_number() {
                fader.value = value.as_u64().unwrap() as u8;
            }
        }
    }
    return Some(fader)
}
