use std::{thread::sleep, time::Duration};
use esp_idf_svc::hal::prelude::*;
use waveshare_esp32_s3_lcd_7_bsp_rs::{http, wifi, slint_platform};

slint::include_modules!();

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().expect("Failed to take peripherals");

    let touch_i2c = esp_idf_svc::hal::i2c::I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio8,
        peripherals.pins.gpio9,
        &esp_idf_svc::hal::i2c::config::Config::new().baudrate(400_000.Hz()),
    ).expect("Failed to create I2C driver");

    slint_platform::init(touch_i2c);

    let mut timer = esp_idf_svc::hal::timer::TimerDriver::new(peripherals.timer00
    , &Default::default()).expect("Failed to create timer");

    let window = MainWindow::new().unwrap();
    let window_handle = window.as_weak();

    window.run().unwrap();
}
