// Local modules

// Local imports

use chrono::{DateTime, Utc};
// Public imports
use rand::prelude::*;
use std::{thread, time::{self, SystemTime}};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct PotHealth {
    ts: String,
    temperature: f32,
    humidity: f32,
    pressure: f32,
    moisture: f32,
    light: f32
}


fn main() {
    let sleep_duration = time::Duration::from_secs(1);
    let mut rng = thread_rng();

    loop {
        // Timestamp creation
        let timestamp = SystemTime::now();
        let timestamp: DateTime<Utc> = timestamp.into();
        let timestamp_as_str = timestamp.format("%Y-%m-%dT%T");

        // Generate sensor values
        let temperature_celsius: f32 = rng.gen::<f32>() * 30.0;
        let humidity: f32 = rng.gen();
        let pressure_bar: f32 = rng.gen();
        let moisture_level: f32 = rng.gen::<f32>() * 10_000.0;
        let light_level: f32 = rng.gen::<f32>() * 20_000.0;
        
        let message = PotHealth{
            ts: timestamp_as_str.to_string(),
            temperature: temperature_celsius,
            humidity: humidity,
            pressure: pressure_bar,
            moisture: moisture_level,
            light: light_level
        };

        let serialized = serde_json::to_string(&message);

        println!("{}", serialized.unwrap());

        thread::sleep(sleep_duration)
    }
}