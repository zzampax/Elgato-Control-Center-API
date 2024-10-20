use serde::{Deserialize, Serialize};

// Lightstrip struct
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct LightStrip {
    pub on: u32,
    pub hue: f32,
    pub saturation: f32,
    pub brightness: u32,
}

// Keylight struct
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Keylight {
    pub on: u32,
    pub brightness: u32,
    pub temperature: i32,
}

// Enum to encapsulate both types
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Light {
    LightStrip(LightStrip),
    Keylight(Keylight),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightBucket {
    #[serde(rename = "numberOfLights")]
    pub number_of_lights: u32,
    pub lights: Vec<Light>,
}

impl LightStrip {
    pub fn toggle(&mut self) {
        self.on = if self.on == 1 { 0 } else { 1 }
    }

    pub fn set_brightness(&mut self, brightness: u32) {
        self.brightness = brightness;
    }

    pub fn set_color(&mut self, hue: f32, saturation: f32) {
        self.hue = hue;
        self.saturation = saturation;
    }

    pub fn get_status(&self) -> serde_json::Value {
        serde_json::json!({
            "on": self.on,
            "brightness": self.brightness,
            "hue": self.hue,
            "saturation": self.saturation,
        })
    }
}

impl Keylight {
    pub fn toggle(&mut self) {
        self.on = if self.on == 1 { 0 } else { 1 }
    }

    pub fn set_brightness(&mut self, brightness: u32) {
        self.brightness = brightness;
    }

    pub fn set_temperature(&mut self, temperature: i32) {
        self.temperature = temperature;
    }

    pub fn get_status(&self) -> serde_json::Value {
        serde_json::json!({
            "on": self.on,
            "brightness": self.brightness,
            "temperature": self.temperature,
        })
    }
}
