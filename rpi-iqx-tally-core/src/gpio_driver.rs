use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;
use crate::json_handler::*;

pub fn init_gpio(gpio: &Gpio, tally_cfg: &TallyConfig) -> Result<Vec<OutputPin>, Box<dyn Error>> {
    let mut tally_pins: Vec<OutputPin> = vec![];
    for tally in &tally_cfg.tallys {
        let mut gpio_handler = match gpio.get(tally.gpio) {
            Ok(pin) => pin.into_output(),
            Err(e) => return Err(Box::new(e)),
        };
        tally_pins.push(gpio_handler);
    }
    Ok(tally_pins)
}

pub fn reset_all_gpio(pins: &mut Vec<OutputPin>) {
    for pin in pins {
        println!("{}", pin.pin());
        pin.set_low();
    }
}
