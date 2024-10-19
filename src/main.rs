use clap::{command, Arg};
use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Light {
    on: i8,
    hue: f32,
    saturation: f32,
    brightness: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Lights {
    #[serde(rename = "numberOfLights")]
    number_of_lights: usize,
    lights: Vec<Light>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ElgatoLight {
    ip: String,
    port: u16,
    number_of_lights: usize,
    lights: Vec<Light>,
}

impl ElgatoLight {
    fn new(ip: String, port: u16) -> Self {
        let url = format!("http://{}:{}/elgato/lights", ip, port);
        // get the data from the lightstrip as a JSON object
        let response = Client::new()
            .get(&url)
            .send()
            .expect("Failed to send request");
        let lights: Lights = response.json().expect("Failed to parse JSON");
        ElgatoLight{
            ip,
            port,
            number_of_lights: lights.number_of_lights,
            lights: lights.lights,
        }
    }
    fn set_on(&mut self, on: i8) {
        for light in self.lights.iter_mut() {
            light.on = on;
        }
        self.put_light();
    }
    fn set_hue(&mut self, hue: f32) {
        for light in self.lights.iter_mut() {
            light.hue = hue;
        }
        self.put_light();
    }
    fn set_saturation(&mut self, saturation: f32) {
        for light in self.lights.iter_mut() {
            light.saturation = saturation;
        }
        self.put_light();
    }
    fn set_brightness(&mut self, brightness: i32) {
        for light in self.lights.iter_mut() {
            light.brightness = brightness;
        }
        self.put_light();
    }
    fn put_light(&mut self) {
        let payload = Lights {
            number_of_lights: self.number_of_lights,
            lights: self.lights.clone(),
        };
        let url = format!("http://{}:{}/elgato/lights", self.ip, self.port);
        let response = Client::new()
            .put(&url)
            .json(&payload)
            .send()
            .expect("Failed to send request");
        let lights: Lights = response.json().expect("Failed to parse JSON");
        self.lights = lights.lights;
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let matches = command!()
        .name("Control Elgato Lightstrip")
        .version("0.1.0")
        .about("Control Elgato Lightstrip")
        .arg(
            Arg::new("ip")
                .long("ip")
                .help("Elgato Lightstrip IP address")
                .required(true),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .help("Elgato Lightstrip port")
                .required(true),
        )
        .arg(
            Arg::new("on")
                .long("on")
                .help("Turn on the lightstrip"),
        )
        .arg(
            Arg::new("off")
                .long("off")
                .help("Turn off the lightstrip"),
        )
        .arg(
            Arg::new("hue")
                .long("hue")
                .help("Set the hue of the lightstrip"),
        )
        .arg(
            Arg::new("saturation")
                .long("saturation")
                .help("Set the saturation of the lightstrip"),
        )
        .arg(
            Arg::new("brightness")
                .long("brightness")
                .help("Set the brightness of the lightstrip"),
        )
        .get_matches();

    let ip: String = matches.get_one::<String>("ip").unwrap().clone();
    let port_str: String = matches
        .get_one::<String>("port")
        .expect("`port` is required")
        .clone();
    let port: usize = port_str.parse().expect("`port` must be a valid number");

    println!("{}:{}", ip, port);
    let mut lights = ElgatoLight::new(ip, port as u16);

    if matches.contains_id("on") {
        lights.set_on(1);
    }
    if matches.contains_id("off") {
        lights.set_on(0);
    }
    if let Some(hue) = matches.get_one::<f32>("hue") {
        lights.set_hue(*hue);
    }
    if let Some(saturation) = matches.get_one::<f32>("saturation") {
        lights.set_saturation(*saturation);
    }
    if let Some(brightness) = matches.get_one::<i32>("brightness") {
        lights.set_brightness(*brightness);
    }

    println!("{:?}", lights);

    Ok(())
}
