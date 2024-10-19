# ElgatoControlCenter-API

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
## Payload Examples
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