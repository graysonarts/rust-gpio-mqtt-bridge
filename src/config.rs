use rumqttc::MqttOptions;
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;

// #[cfg(target = "armv7-unknown-linux-gnueabihf")]
use rppal::gpio::Trigger;

// #[cfg(not(target = "armv7-unknown-linux-gnueabihf"))]
// use crate::fake::Trigger;

#[derive(Debug, Deserialize)]
pub struct MqttConfig {
    pub host: String,
    pub username: Option<String>,
    pub(crate) password: Option<String>,
    pub topic_root: Option<String>,
}

impl MqttConfig {
    pub fn as_mqtt_options(&self) -> MqttOptions {
        let mut opts = MqttOptions::new("waterland-ctrl", &self.host, 1883);
        match (self.username.as_ref(), self.password.as_ref()) {
            (Some(username), Some(password)) => {
                opts.set_credentials(username, password);
            }
            _ => {}
        };

        opts
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum TriggerType {
    #[serde(rename = "rising")]
    Rising,
    #[serde(rename = "falling")]
    Falling,
    #[serde(rename = "transition")]
    AnyTransition,
}

impl Into<Trigger> for TriggerType {
    fn into(self) -> Trigger {
        use TriggerType::*;
        match self {
            Rising => Trigger::RisingEdge,
            Falling => Trigger::FallingEdge,
            AnyTransition => Trigger::Both,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GpioConfig {
    pub pin: u8,
    pub topic: String,
    pub trigger: TriggerType,
    with_pullup: Option<bool>,
}

impl GpioConfig {
    pub fn new(pin: u8, topic: &str, trigger: TriggerType) -> Self {
        Self {
            pin,
            topic: topic.to_owned(),
            trigger,
            with_pullup: None,
        }
    }
    pub fn with_pullup(&self) -> bool {
        self.with_pullup.unwrap_or(false)
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub gpio: Vec<GpioConfig>,
}

#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    ParseError(toml::de::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::IoError(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::ParseError(e)
    }
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let config_data = fs::read_to_string(path)?;

        Ok(toml::from_str(&config_data)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_load_config() {
        let config = Config::from_file("config.toml").expect("config.toml should load");
        assert_eq!(config.mqtt.host, "broker.hivemq.com");
        assert_eq!(config.mqtt.topic_root, Some("waterland".to_owned()));
        assert_eq!(config.mqtt.username, None);
        assert_eq!(config.mqtt.password, None);
        assert_eq!(config.gpio.len(), 3);
        assert_eq!(config.gpio[0].pin, 23);
        assert!(config.gpio[0].with_pullup());
        assert_eq!(config.gpio[1].pin, 24);
        assert!(!config.gpio[1].with_pullup());
        assert_eq!(config.gpio[2].pin, 25);
    }
}
