// Local modules

// Local imports
use atmospheric_sensor;
use light_sensor;
use moisture_sensor;

// Public imports
use linux_embedded_hal::I2cdev;
use chrono::offset::Utc;
use chrono::DateTime;
use std::{thread, time, time::SystemTime};


fn main() {
    // Load sensors

    let mut sensor_bme280 = atmospheric_sensor::AtmosphericSensor::build(
        I2cdev::new("/dev/i2c-1").unwrap(),
        atmospheric_sensor::Address::Alternative
    );

    let mut sensor_moisture = moisture_sensor::MoistureSensor::build(
        I2cdev::new("/dev/i2c-1").unwrap(),
        moisture_sensor::Address::Default
    );

    let mut sensor_veml6030 = light_sensor::LightSensor::build(
        I2cdev::new("/dev/i2c-1").unwrap(),
        light_sensor::Address::Default.into()
    );


    thread::sleep(time::Duration::from_secs(1));

    loop {
        let timestamp = SystemTime::now();
        let timestamp: DateTime<Utc> = timestamp.into();
        let timestamp = timestamp.format("%Y-%m-%dT%T");

        let temp = sensor_bme280.get_temperature_celsius().unwrap();
        let hum = sensor_bme280.get_humidity_relative().unwrap();
        let press = sensor_bme280.get_pressure_pascal().unwrap();

        println!("[{timestamp}] temperature={temp}, humididy={hum}, pressure={press}");

        let moisture_level = sensor_moisture.get_moisture_level().unwrap();

        println!("[{timestamp}] moisture_level={moisture_level}");

        let lux = sensor_veml6030.get_ambient_light_lux().unwrap();

        println!("[{timestamp}] lux={lux}");

        thread::sleep(time::Duration::from_secs(3));
    }
}
