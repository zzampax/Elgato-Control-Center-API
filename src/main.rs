use clap::{command, Arg};
use client::ElgatoClient;
use std::error::Error;

mod client;
mod devices;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = command!()
        .name("Elgato Control CLI")
        .version("0.1.0")
        .about("Elgato Control CLI")
        .arg(
            Arg::new("ip")
                .long("ip")
                .help("Elgato KeyLight/light_clienttrip IP address")
                .required(true),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .help("Elgato KeyLight/light_clienttrip port")
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
                .help("Set the hue of the light_clienttrip"),
        )
        .arg(
            Arg::new("saturation")
                .long("saturation")
                .help("Set the saturation of the light_clienttrip"),
        )
        .arg(
            Arg::new("temperature")
                .long("temperature")
                .help("Set the temperature of the keylight"),
        )
        .arg(
            Arg::new("brightness")
                .long("brightness")
                .help("Set the brightness of the light"),
        )
        .get_matches();

    let ip: String = matches.get_one::<String>("ip").unwrap().clone();
    let port_str: String = matches
        .get_one::<String>("port")
        .expect("`port` is required")
        .clone();
    let port: usize = port_str.parse().expect("`port` must be a valid number");

    let mut light_client = ElgatoClient::new(&ip, port as u16);
    // If the `on` or `off` flags are present, toggle the lightstrip
    if matches.contains_id("on") || matches.contains_id("off") {
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

    println!("{:?}", light_client);

    Ok(())
}
