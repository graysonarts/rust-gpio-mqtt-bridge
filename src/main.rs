use rppal::system::DeviceInfo;

use colored::Colorize;
use std::error::Error;

use rumqttc::{Client, QoS};
use tokio::task;

use gpio_mqtt_bridge::config::Config;
use gpio_mqtt_bridge::InterruptCtrl;

use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

macro_rules! info {
    () => (print!("\n"));
    ($($arg:tt)*) => ({
        let msg = format!($($arg)*);
        println!("{}", msg.truecolor(64, 64, 64));
    })
}

macro_rules! bright {
    () => (print!("\n"));
    ($($arg:tt)*) => ({
        let msg = format!($($arg)*);
        println!("{}", msg.truecolor(128, 128, 128));
    })
}

macro_rules! error {
    () => (print!("\n"));
    ($($arg:tt)*) => ({
        let msg = format!($($arg)*);
        println!("{}", msg.red());
    })
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let dev_info = DeviceInfo::new()?;

    info!(
        "{}/{} waterland {}",
        dev_info.model(),
        dev_info.soc(),
        VERSION
    );

    let config = Config::from_file("config.toml")?;
    let topic_root = config
        .mqtt
        .topic_root
        .as_ref()
        .unwrap_or(&"gpio".to_string())
        .to_string();

    info!("Configuring pins for input");
    let ctrl = InterruptCtrl::from_gpio_config(&config.gpio)?;

    info!("Connecting to MQTT Broker {}", config.mqtt.host);
    let mut mqttopts = config.mqtt.as_mqtt_options();
    mqttopts.set_keep_alive(5);
    let (mut client, mut connection) = Client::new(mqttopts, 10);

    thread::spawn(move || loop {
        let result = ctrl.poll(|subtopic, message| {
            let topic = format!("{}/{}", topic_root, subtopic);
            match client.publish(&topic, QoS::AtLeastOnce, false, message) {
                Ok(_) => bright!("{} => {}", topic, message),
                Err(e) => error!("Failed to send {}/{}: {}", topic, message, e),
            };
        });

        if let Err(e) = result {
            error!("Error Polling Pins: {}", e);
        }
    });

    loop {
        for (i, notification) in connection.iter().enumerate() {
            info!("MQTT notification: {:?}", notification);
        }
    }
}
