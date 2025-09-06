#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        return Self {
            alias: alias.to_string(),
            brightness: 0,
        };
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    if let Some(light) = lights.iter_mut().find(|l| l.alias == alias) {
        light.brightness = value;
    }
}
