use arduino_hal::port::{mode, Pin};

pub struct Lcd {
    rs: Pin<mode::Output>,
    en: Pin<mode::Output>,
    d4: Pin<mode::Output>,
    d5: Pin<mode::Output>,
    d6: Pin<mode::Output>,
    d7: Pin<mode::Output>,
}

impl Lcd {
    pub fn new() -> Self {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);

        let rs = pins.d1.into_output().downgrade();
        let en = pins.d2.into_output().downgrade();
        let d4 = pins.d4.into_output().downgrade();
        let d5 = pins.d5.into_output().downgrade();
        let d6 = pins.d6.into_output().downgrade();
        let d7 = pins.d7.into_output().downgrade();

        Lcd {
            rs: rs,
            en: en,
            d4: d4,
            d5: d5,
            d6: d6,
            d7: d7,
        }
    }

    pub fn init(&mut self) {
        // Initialization sequence
        for _ in 0..3 {
            self.write_bit(0x03);
            arduino_hal::delay_ms(5);
        }
        self.write_bit(0x02);
        arduino_hal::delay_ms(5);

        // Function set
        self.write_command(0x28);
        arduino_hal::delay_ms(5);

        // Display on/off control
        self.write_command(0x0C);
        arduino_hal::delay_ms(5);

        // Clear display
        self.write_command(0x01);
        arduino_hal::delay_ms(5);

        // Entry mode set
        self.write_command(0x06);
        arduino_hal::delay_ms(5);
    }

    pub fn write_command(&mut self, command: u8) {
        self.write_byte(command, false);
    }

    pub fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            self.write_byte(c as u8, true);
            arduino_hal::delay_ms(1);
        }
    }

    pub fn write_byte(&mut self, byte: u8, rs: bool) {
        if rs {
            self.rs.set_high();
        } else {
            self.rs.set_low();
        }
        // First high bits
        self.write_bit(byte >> 4);
        // Then low bits
        self.write_bit(byte & 0x0F);
    }

    fn write_bit(&mut self, byte: u8) {
        if (byte & 0x01) != 0 {
            self.d4.set_high();
        } else {
            self.d4.set_low();
        }
        if (byte & 0x02) != 0 {
            self.d5.set_high();
        } else {
            self.d5.set_low();
        }
        if (byte & 0x04) != 0 {
            self.d6.set_high();
        } else {
            self.d6.set_low();
        }
        if (byte & 0x08) != 0 {
            self.d7.set_high();
        } else {
            self.d7.set_low();
        }
        self.enable();
    }

    fn enable(&mut self) {
        self.en.set_high();
        arduino_hal::delay_us(1);
        self.en.set_low();
        arduino_hal::delay_us(1);
    }
}
