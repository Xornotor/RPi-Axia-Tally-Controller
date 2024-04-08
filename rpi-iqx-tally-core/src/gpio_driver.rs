use crate::event_parser::*;
use crate::json_handler::*;
use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;
use std::sync::mpsc::Receiver;

pub fn init_gpio(gpio: &Gpio, tally_cfg: &TallyConfig) -> Result<Vec<OutputPin>, Box<dyn Error>> {
    let mut tally_pins: Vec<OutputPin> = vec![];
    for tally in &tally_cfg.tallys {
        let gpio_handler = match gpio.get(tally.gpio_relay) {
            Ok(pin) => pin.into_output(),
            Err(e) => return Err(Box::new(e)),
        };
        tally_pins.push(gpio_handler);
    }
    Ok(tally_pins)
}

// Relays active on low state. Reset function puts the outputs on high state.
pub fn reset_all_gpio(pins: &mut Vec<OutputPin>) {
    for pin in pins {
        pin.set_high();
    }
}

pub fn decode_receivers_to_gpio(
    receivers: &Vec<Receiver<String>>,
    tally_cfg: &TallyConfig,
    tally_pins: &mut Vec<OutputPin>,
) {
    for receiver in receivers {
        let msg = match receiver.try_recv() {
            Ok(recv_msg) => recv_msg,
            Err(_) => continue,
        };

        println!("{msg}");

        let info_tuple = match parse_event(&msg) {
            Some(tuple) => tuple,
            None => continue,
        };

        let (console_number, fader_number, state) = info_tuple;

        let mut tally_pin = 0;
        for tally in &tally_cfg.tallys {
            if tally.id_console == console_number && tally.id_fader == fader_number {
                if tally.enable {
                    tally_pin = tally.gpio_relay;
                }
                break;
            }
        }
        if tally_pin != 0 {
            for pin in &mut tally_pins.iter_mut() {
                if pin.pin() == tally_pin {
                    if state {
                        pin.set_low();
                    } else {
                        pin.set_high();
                    }
                    break;
                }
            }
        }
    }
}
