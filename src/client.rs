use crate::devices::{Light, LightBucket};
use reqwest::blocking::Client;
use log::debug;
use std::process;

#[derive(Clone, Debug)]
pub struct ElgatoClient {
    endpoint: String,
    client: reqwest::blocking::Client,
    pub light: Light,
}

impl Light {
    pub fn get_status(&self) -> serde_json::Value {
        match self {
            Light::Keylight(light) => light.get_status(),
            Light::LightStrip(light) => light.get_status(),
        }
    }
}

impl ElgatoClient {
    pub fn new(ip: &str, port: u16) -> Self {
        let client = Client::new();
        let endpoint = format!("http://{ip}:{port}/elgato/lights");

        let response = client.get(endpoint.clone()).send().unwrap();
        let light_bucket: LightBucket = response.json().unwrap();

        Self {
            endpoint,
            client,
            light: light_bucket.lights[0].clone(),
        }
    }

    // toString() method calls get_status() method
    pub fn to_string(&self) -> String {
        self.light.get_status().to_string()
    }

    pub fn toggle(&mut self) {
        match &self.light {
            Light::Keylight(light) => {
                let mut light = light.clone();
                light.toggle();
                self.light = Light::Keylight(light);
            }
            Light::LightStrip(light) => {
                let mut light = light.clone();
                light.toggle();
                self.light = Light::LightStrip(light);
            }
        }

        self.client
            .put(&self.endpoint)
            .json(&self.prepare_payload())
            .send()
            .unwrap();
    }

    pub fn set_brightness(&mut self, brightness: u32) {
        match &self.light {
            Light::Keylight(light) => {
                let mut light = light.clone();
                light.set_brightness(brightness);
                self.light = Light::Keylight(light);
            }
            Light::LightStrip(light) => {
                let mut light = light.clone();
                light.set_brightness(brightness);
                self.light = Light::LightStrip(light);
            }
        }

        self.client
            .put(&self.endpoint)
            .json(&self.prepare_payload())
            .send()
            .unwrap();
    }

    pub fn set_temperature(&mut self, temperature: i32) {
        match &self.light {
            Light::Keylight(light) => {
                let mut light = light.clone();
                light.set_temperature(temperature);
                self.light = Light::Keylight(light);
            }
            Light::LightStrip(_light) => {
                println!("{}", serde_json::json!({"error": "Temperature is not supported for LightStrip"}).to_string());
                process::exit(1);
            }
        }

        self.client
            .put(&self.endpoint)
            .json(&self.prepare_payload())
            .send()
            .unwrap();
    }

    pub fn set_color(&mut self, saturation: f32, hue: f32) {
        match &self.light {
            Light::Keylight(_) => {
                println!("{}", serde_json::json!({"error": "Color is not supported for Keylight"}).to_string());
                process::exit(1);
            }
            Light::LightStrip(light) => {
                let mut light = light.clone();
                light.set_color(hue, saturation);
                self.light = Light::LightStrip(light);
            }
        }

        let response = self
            .client
            .put(&self.endpoint)
            .json(&self.prepare_payload())
            .send()
            .unwrap();

        debug!("{:?}", response.text());
    }

    fn prepare_payload(&self) -> LightBucket {
        LightBucket {
            number_of_lights: 1,
            lights: vec![self.light.clone()],
        }
    }
}
