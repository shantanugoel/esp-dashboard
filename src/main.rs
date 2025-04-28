use esp_idf_svc::hal::prelude::*;
use serde_json::Value;
use slint::SharedString;
use waveshare_esp32_s3_lcd_7_bsp_rs::{http, wifi, slint_platform};

slint::include_modules!();

#[toml_cfg::toml_config]
struct AppConfig {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_password: &'static str,
}

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

    let window = MainWindow::new().unwrap();
    let window_handle = window.as_weak();

    let event_loop = esp_idf_svc::eventloop::EspSystemEventLoop::take().expect("Failed to create event loop");
    let nvs_partition = esp_idf_svc::nvs::EspDefaultNvsPartition::take().expect("Failed to create NVS partition");
    let mut wifi_driver = wifi::init(peripherals.modem, event_loop, Some(nvs_partition)).expect("Failed to create wifi driver");
    wifi::connect(&mut wifi_driver, APP_CONFIG.wifi_ssid, APP_CONFIG.wifi_password).expect("Failed to connect to wifi");
    wifi::wait_for_connection(&mut wifi_driver).expect("Failed to wait for connection");

    let mut client = http::HttpClient::new().expect("Failed to create http client");
    let headers = [("Content-Type", "application/json")];
    let url = "https://api.chucknorris.io/jokes/random";

    window.on_update_fact(move || {
        let window = window_handle.upgrade().unwrap();
        let body = client.get(url, &headers);
        let fact = if let Ok(body) = body {
            let v: Value = serde_json::from_str(&body).expect("Failed to parse json");
            v["value"].as_str().unwrap_or("No fact found").to_string()
        } else {
            "Couldn't get fact. Check your connection or try again.".to_string()
        };
        window.set_fact(SharedString::from(fact));
    });

    window.run().unwrap();
}
