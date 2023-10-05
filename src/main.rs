#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use arduino_hal::spi;
use display_interface_spi::SPIInterface;
use display_interface_spi::SPIInterfaceNoCS;
use arduino_hal::Delay;

use mipidsi::Builder;
use mipidsi::Orientation;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RgbColor;

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    //let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let (mut spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );

    //let di = SPIInterface::new(spi, pins.d8.into_output(), pins.d7.into_output());
    let di = SPIInterfaceNoCS::new(spi, pins.d8.into_output());
    let mut delay = Delay::new();

    let mut display = Builder::st7789(di)
        .with_display_size(240, 240)
        .with_framebuffer_size(240, 240)
        .with_orientation(Orientation::LandscapeInverted(true))
        .init(&mut delay, Some(pins.d9.into_output()))
        .unwrap();

    display.clear(Rgb565::RED).unwrap();

    loop {}
}
