use clap::{command, Arg};
use client::{Config, ElgatoClient};
use std::error::Error;
use std::process;

mod client;
mod devices;

fn load_config(optional_path: Option<&String>) -> Result<Config, Box<dyn Error>> {
    let homedir = std::env::var("HOME").map_err(|_| "Failed to read $HOME environment variable")?;

    let path = match optional_path {
        Some(path) => path.to_string(),
        None => format!("{}/.config/elgato-control-center/config.toml", homedir),
    };

    let config = Config::new(path.as_str())
        .ok_or_else(|| format!("Failed to load configuration from path: {}", path))?;

    Ok(config)
}

fn matches() -> clap::ArgMatches {
    return command!()
        .name("Elgato Control CLI")
        .version("1.1.1")
        .about("Elgato Control CLI")
        .arg(
            Arg::new("ip")
                .long("ip")
                .help("Elgato KeyLight/LightStrip IP address")
                .required(false),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .help("Elgato KeyLight/LightStrip port")
                .required(false),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .help("Config file providing ip and port, default path ~/.config/elgato-control-center/config.toml")
                .required(false),
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
    let config = load_config(matches.get_one::<String>("config"));

    let ip: String = match matches.get_one::<String>("ip") {
        Some(port) => port.clone(),
        None => match &config {
            Ok(config) => config.device.ip.clone(),
            Err(_) => {
                println!("Error retrieving IP, exiting.");
                process::exit(1);
            }
        },
    };

    let port: u16 = match matches.get_one::<String>("port") {
        Some(port) => match port.clone().parse() {
            Ok(port) => port,
            Err(_) => {
                println!("Error handling Port, exiting.");
                process::exit(2);
            }
        },
        None => match &config {
            Ok(config) => config.device.port.clone(),
            Err(_) => {
                println!("Error retrieving Port, exiting.");
                process::exit(1);
            }
        },
    };

    let mut light_client = ElgatoClient::new(&ip, port);
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
