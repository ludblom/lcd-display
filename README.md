`lcd-display`
==================
A simple "Hello World!" project with an lcd. Supports the boards:

 - Arduino Leonardo
 - Arduino Mega 2560
 - Arduino Mega 1280
 - Arduino Nano
 - Arduino Nano New Bootloader (Manufactured after January 2018)
 - Arduino Uno
 - SparkFun ProMicro
 - Adafruit Trinket
 - Adafruit Trinket Pro

## Usage
If you don't have them already, install [`cargo-generate`] and [`ravedude`]:

```bash
cargo install cargo-generate
cargo install ravedude
```

You will be prompted to select your board - do so and you're ready to roll!
Everything is prepared so you should be able to just

```bash
cargo run
```

and see a blinky flashed to your board!

## Props
The template for this project comes from [`rahix`].

[`rahix`]: https://github.com/Rahix/avr-hal-template.git
[`cargo-generate`]: https://github.com/cargo-generate/cargo-generate
[`ravedude`]: https://github.com/Rahix/avr-hal/tree/main/ravedude

## License
Licensed under

 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
