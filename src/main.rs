#![no_std]
#![no_main]

use panic_halt as _;

mod lcd;
use lcd::Lcd;

#[arduino_hal::entry]
fn main() -> ! {
    let mut lcd = Lcd::new();
    lcd.init();
    lcd.write_string("Hello World!");
    loop {}
}
