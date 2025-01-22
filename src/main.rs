#![no_std]
#![no_main]

use ch32_hal as hal;
use ch32_hal::gpio::Speed;
use hal::delay::Delay;
use hal::gpio::{Level, Output};
use panic_halt as _;

#[qingke_rt::entry]
fn main() -> ! {
    let config = hal::Config::default();

    let peripherals = hal::init(config);

    let mut led = Output::new(peripherals.PD1, Level::Low, Speed::Low);

    let mut delay = Delay;

    loop {
        led.toggle();

        delay.delay_ms(1000);
    }
}
