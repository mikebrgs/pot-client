// Local modules

// Local imports

// Public imports
use rand::prelude::*;
use scopeguard::defer;
use std::{thread, time::{self, SystemTime}};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use paho_mqtt as mqtt;


#[derive(Serialize, Deserialize, Debug)]
struct PotHealth {
    ts: DateTime<Utc>,
    device_id: String,
    temperature: f32,
    humidity: f32,
    pressure: f32,
    moisture: f32,
    light: f32
}


fn main() {
    let sleep_duration = time::Duration::from_secs(1);
    let mut rng = thread_rng();
    let mut cli = mqtt::Client::new(
        "mqtt://localhost:1883".to_string()
    ).unwrap();
    cli.set_timeout(time::Duration::from_secs(5));
    let _ = cli.connect(None).unwrap();
    defer! {
        cli.disconnect(None).unwrap();
    }

    loop {
        // Timestamp creation
        let timestamp = SystemTime::now();
        let timestamp: DateTime<Utc> = timestamp.into();

        // Generate sensor values
        let temperature_celsius: f32 = rng.gen::<f32>() * 30.0;
        let humidity: f32 = rng.gen();
        let pressure_bar: f32 = rng.gen();
        let moisture_level: f32 = rng.gen::<f32>() * 10_000.0;
        let light_level: f32 = rng.gen::<f32>() * 20_000.0;
        
        let pot_health = PotHealth{
            ts: timestamp,
            device_id: "mock".to_string(),
            temperature: temperature_celsius,
            humidity: humidity,
            pressure: pressure_bar,
            moisture: moisture_level,
            light: light_level
        };

        let serialized = serde_json::to_string(&pot_health).unwrap();
        let message = mqtt::Message::new(
            "pot/health",
            serialized.clone(),
            0
        );

        let _ = cli.publish(message).unwrap();

        println!("{}", serialized);

        thread::sleep(sleep_duration)
    }

}