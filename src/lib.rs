pub mod config;

use rppal::{gpio::Gpio, gpio::InputPin};

use config::GpioConfig;
use serde_derive::Serialize;
use std::collections::HashMap;

#[derive(Debug)]
pub struct InterruptCtrl {
    gpio: Gpio,
    pins: Vec<InputPin>,
    topic_map: HashMap<u8, String>,
}

#[derive(Debug)]
pub enum ICError {
    GenericError(String),
}

impl std::error::Error for ICError {}

impl From<rppal::gpio::Error> for ICError {
    fn from(gpioe: rppal::gpio::Error) -> ICError {
        ICError::GenericError(format!("{:?}", gpioe))
    }
}

impl std::fmt::Display for ICError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use ICError::*;
        match self {
            GenericError(e) => write!(f, "InterruptCtrl Error: {}", e),
        }
    }
}

#[derive(Debug, Serialize)]
struct Message {
    pin: u8,
    message: String,
}

impl Message {
    pub fn new(pin: u8, message: String) -> Message {
        Message { pin, message }
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "Unknown Payload".to_string())
    }
}

impl InterruptCtrl {
    pub fn from_gpio_config(configs: &[GpioConfig]) -> Result<Self, ICError> {
        let gpio = Gpio::new()?;

        let pins: Result<Vec<InputPin>, _> = configs
            .iter()
            .map(|c| {
                gpio.get(c.pin)
                    .map(|p| match c.with_pullup() {
                        true => p.into_input_pullup(),
                        false => p.into_input(),
                    })
                    .map(|mut p| {
                        p.set_interrupt(c.trigger.into())
                            .unwrap_or_else(|_| panic!());
                        p
                    })
            })
            .collect();
        let topics: HashMap<u8, String> = configs
            .iter()
            .map(|c| (c.pin, c.topic.to_string()))
            .collect();

        let pins = pins?;
        let retval = InterruptCtrl {
            gpio,
            pins,
            topic_map: topics,
        };

        Ok(retval)
    }

    pub fn poll<C>(&self, mut callback: C) -> Result<(), ICError>
    where
        C: FnMut(&str, &str),
    {
        let pin_refs: Vec<_> = self.pins.iter().collect();
        let result = self.gpio.poll_interrupts(&pin_refs, false, None)?; // TODO: Maybe implement a timeout
        if let Some((pin, level)) = result {
            let topic = self.topic_map.get(&pin.pin());
            if let Some(topic) = topic {
                let message = Message::new(pin.pin(), format!("{:?}", level)).to_string();
                callback(topic, &message);
            } else {
                unreachable!();
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::config::TriggerType;

    use super::*;

    #[test]
    fn can_create_interrupt_ctrl() {
        let gpios = vec![GpioConfig::new(1, "foo", TriggerType::Falling)];
        let ctrl = InterruptCtrl::from_gpio_config(&gpios).expect("Unable to create Ctrl");
    }

    #[test]
    fn cannot_create_same_pin_twice() {
        let gpios = vec![
            GpioConfig::new(1, "yes", TriggerType::Falling),
            GpioConfig::new(1, "no", TriggerType::Rising),
        ];
        let ctrl = InterruptCtrl::from_gpio_config(&gpios);
        assert!(ctrl.is_err());
    }
}
