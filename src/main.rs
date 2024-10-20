use clap::{command, Arg};
use client::ElgatoClient;
use std::error::Error;

mod client;
mod devices;

fn matches() -> clap::ArgMatches {
    return command!()
        .name("Elgato Control CLI")
        .version("1.1.0")
        .about("Elgato Control CLI")
        .arg(
            Arg::new("ip")
                .long("ip")
                .help("Elgato KeyLight/LightStrip IP address")
                .required(true),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .help("Elgato KeyLight/LightStrip port")
                .required(true),
        )
        .arg(
            Arg::new("toggle")
                .long("toggle")
                .help("Turn on/off the Elgato KeyLight/LightStrip")
                .num_args(0),
        )
        .arg(
            Arg::new("hue")
                .long("hue")
                .help("Set the hue of the Elgato LightStrip"),
        )
        .arg(
            Arg::new("saturation")
                .long("saturation")
                .help("Set the saturation of the Elgato LightStrip"),
        )
        .arg(
            Arg::new("temperature")
                .long("temperature")
                .help("Set the temperature of the Elgato KeyLight"),
        )
        .arg(
            Arg::new("brightness")
                .long("brightness")
                .help("Set the brightness of the Elgato KeyLight/LightStrip"),
        )
        .get_matches();
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches: clap::ArgMatches = matches();

    let ip: String = matches.get_one::<String>("ip").unwrap().clone();
    let port_str: String = matches
        .get_one::<String>("port")
        .expect("`port` is required")
        .clone();
    let port: usize = port_str.parse().expect("`port` must be a valid number");

    let mut light_client = ElgatoClient::new(&ip, port as u16);
    // If the `on` or `off` flags are present, toggle the LightStrip
    if matches.value_source("toggle") == Some(clap::parser::ValueSource::CommandLine) {
        light_client.toggle();
    }
    // If the `brightness` flag is present, set the brightness of the lightsttrip
    if matches.contains_id("hue") && matches.contains_id("saturation") {
        let hue_str: String = matches.get_one::<String>("hue").unwrap().clone();
        let hue: f32 = hue_str.parse().expect("`hue` must be a valid number");

        let saturation_str: String = matches.get_one::<String>("saturation").unwrap().clone();
        let saturation: f32 = saturation_str
            .parse()
            .expect("`saturation` must be a valid number");

        light_client.set_color(saturation, hue);
    }
    // If the `temperature` flag is present, set the temperature of the keylight
    if matches.contains_id("temperature") {
        let temperature_str: String = matches.get_one::<String>("temperature").unwrap().clone();
        let temperature: i32 = temperature_str
            .parse()
            .expect("`temperature` must be a valid number");

        light_client.set_temperature(temperature);
    }
    // If the `brightness` flag is present, set the brightness of the lightsttrip
    if matches.contains_id("brightness") {
        let brightness_str: String = matches.get_one::<String>("brightness").unwrap().clone();
        let brightness: u32 = brightness_str
            .parse()
            .expect("`brightness` must be a valid number");

        light_client.set_brightness(brightness);
    }

    println!("{}", light_client.to_string());

    Ok(())
}
