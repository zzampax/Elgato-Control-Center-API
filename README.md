# Elgato-Control-Center-API

<br>
<div align="center">

![Language](https://img.shields.io/github/languages/top/zzampax/ecc-api.svg?style=for-the-badge&labelColor=black&logo=rust&logoColor=red&label=Rust)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![Github](https://img.shields.io/badge/GitHub-000000?style=for-the-badge&logo=github&logoColor=white)
![WAM](https://img.shields.io/badge/APIs-RULE%20THE%20WORLD-CD3713?style=for-the-badge&labelColor=black)

<img src="https://rust-book.cs.brown.edu/img/ferris/does_not_compile.svg" alt="ECC" height="300px">
</div>
<br>

This project is a lightweight API written in RUST that enables interaction with Elgato devices such as [Elgato LightStrips](https://www.elgato.com/us/en/p/light-strip-pro) or [Elgato KeyLights](https://www.elgato.com/us/en/p/key-light).
## Inspiration
This script is heavly inspired by the [Elgato-Control-Center](https://github.com/DanielHe4rt/elgato-control-center.git) project, also written in Rust, this although aims at providing a straightforward CLI API free of any GUI.
## Usage
The script provides arguments description via `ecc-api --help`:
```bash
Elgato Control CLI

Usage: ecc-api [OPTIONS]

Options:
      --ip <ip>                    Elgato KeyLight/LightStrip IP address
  -p, --port <port>                Elgato KeyLight/LightStrip port
  -c, --config <config>            Config file providing ip and port, default path ~/.config/elgato-control-center/config.toml
      --toggle                     Turn on/off the Elgato KeyLight/LightStrip
      --hue <hue>                  Set the hue of the Elgato LightStrip
      --saturation <saturation>    Set the saturation of the Elgato LightStrip
      --temperature <temperature>  Set the temperature of the Elgato KeyLight
      --brightness <brightness>    Set the brightness of the Elgato KeyLight/LightStrip
  -h, --help                       Print help
  -V, --version                    Print version
```
> Tip
> Pipe the output of `ecc-api` using `jq` for prettier syntax:
> ```bash
> ecc-api | jq
> ```
### Config File
The IP and Port of the device can be provided using a `config.toml` file using the following format:
```toml
[device]
ip = "192.168.X.X"
port = 9123 # Default Elgato port
```
The default path is `~/.config/elgato-control-center`, the user can manually pass the path with the `-c/--config` flag
```bash
ecc-api -c path/to/config.toml
```
Or avoid using the `config.toml` file at all:
```bash
ecc-api --ip 192.168.X.X -p 9123
```
## Payload Examples
The whole comunication is via HTTP using JSON.
### Elgato LightStrip
```json
{
  "numberOfLights": 1,
  "lights": [
    {
      "on": 1,
      "hue": 332.000000,
      "saturation": 81.000000,
      "brightness": 99
    }
  ]
}
```
### Elgato Keylight
```json
{
  "numberOfLights": 1,
  "lights": [
    {
      "on": 1,
      "brightness": 12,
      "temperature": 143
    }
  ]
}
```
