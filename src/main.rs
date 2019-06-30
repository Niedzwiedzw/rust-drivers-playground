mod command_ids;

use rppal::{
    gpio::{
        Gpio,
        OutputPin,
    },
    spi::{
        Bus,
        SlaveSelect,
        Spi,
        Mode,
    },
};
use std::thread;
use std::time::Duration;

struct Ssd1306Display {
    spi_channel: Spi,
    dc_pin: OutputPin,
}

impl Ssd1306Display {
    pub fn new() -> Self {
        // this pin's state tells the device whether it's receiving chunk of data (HIGH) or a command (LOW)
        let dc_pin = Gpio::new()
            .expect("unable to initialize GPIO")
            .get(36)
            .expect("unable to access D/C Pin")
            .into_output();
        let spi_channel = Spi::new(
            Bus::Spi0,
            SlaveSelect::Ss0,
            9999,
            Mode::Mode0,
        ).expect("Unable to initialize device.");

        Self { spi_channel, dc_pin }
    }

    pub fn initialize(&mut self) {
        self.command(command_ids::DISPLAYOFF);
        self.command(command_ids::SETDISPLAYCLOCKDIV);
        self.command(0x80);
        self.command(command_ids::SETMULTIPLEX);
        self.command(0x3F);
        self.command(command_ids::SETDISPLAYOFFSET);
        self.command(0x0);
        self.command(command_ids::SETSTARTLINE | 0x0);
        self.command(command_ids::CHARGEPUMP);
        self.command(0x14);
        self.command(command_ids::MEMORYMODE);
        self.command(0x00);
        self.command(command_ids::SEGREMAP | 0x1);
        self.command(command_ids::COMSCANDEC);
        self.command(command_ids::SETCOMPINS);
        self.command(0x12);
        self.command(command_ids::SETCONTRAST);
        self.command(0xCF);
        self.command(command_ids::SETPRECHARGE);
        self.command(0xF1);
        self.command(command_ids::SETVCOMDETECT);
        self.command(0x40);
        self.command(command_ids::DISPLAYALLON_RESUME);
        self.command(command_ids::NORMALDISPLAY);
        self.command(command_ids::DISPLAYON);
    }
    pub fn command(&mut self, command: u8) {
        self.dc_pin
            .set_low();
        self.spi_channel
            .write(&[command])
            .expect("failed to send the command");
    }
}

fn main() {
    let mut device = Ssd1306Display::new();
    device.initialize();
    device.command(command_ids::DISPLAYALLON);
    thread::sleep(Duration::from_micros(5000000));
    println!("Hello, raspberry!");
}
