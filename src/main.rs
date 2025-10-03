use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_sys as _;

use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_esp32_rmt_driver::{driver::color::LedPixelColorGrb24, LedPixelEsp32Rmt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();

    let rmt_channel = peripherals.rmt.channel0;

    let mut ws2812 =
        LedPixelEsp32Rmt::<RGB8, LedPixelColorGrb24>::new(rmt_channel, peripherals.pins.gpio8)?;

    let mut hue: u8 = 0;

    loop {
        let rgb = hsv2rgb(Hsv {
            hue,
            sat: 255,
            val: 255,
        });

        ws2812.write([RGB8::new(rgb.r, rgb.g, rgb.b)])?;

        hue = hue.wrapping_add(1);
        FreeRtos::delay_ms(5);
    }
}
