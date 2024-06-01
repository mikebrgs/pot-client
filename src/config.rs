// Local modules

// Local imports

// Public imports
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub device_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MqttConfig {
    pub host: String,
    pub port: i16
}