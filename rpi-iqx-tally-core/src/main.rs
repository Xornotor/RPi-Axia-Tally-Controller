mod event_parser;
mod gpio_driver;
mod json_handler;
mod lwcp_handler;

use crate::event_parser::*;
use crate::gpio_driver::*;
use crate::json_handler::*;
use crate::lwcp_handler::*;

use rppal::gpio::Gpio;
use std::sync::mpsc::{self, Receiver};
use std::thread;

fn main() {
    // Gather Tally Config File
    let tally_cfg = init_tally_config().expect("Error accessing tally config file");

    // GPIO Initializing
    let gpio = Gpio::new().expect("Failed while configuring GPIO Handler");
    let mut tally_pins = init_gpio(&gpio, &tally_cfg).expect("Failed to initialize GPIO");
    reset_all_gpio(&mut tally_pins);

    // Socket threads and channels (communication with Axia Consoles)
    let mut receivers: Vec<Receiver<String>> = vec![];
    for console in tally_cfg.consoles {
        let (tx, rx) = mpsc::channel();
        receivers.push(rx);
        thread::spawn(move || {
            let mut stream = open_socket(console.ip_addr).unwrap();
            loop {
                let reading = read_from_socket(&mut stream).unwrap();
                for line in reading {
                    tx.send(format!("Console={} {}", console.id_console, line));
                }
            }
        });
    }

    // Continuous message receiving and GPIO management
    loop {
        for receiver in &receivers {
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
                        tally_pin = tally.gpio;
                    }
                    break;
                }
            }
            if tally_pin != 0 {
                for pin in &mut tally_pins {
                    if pin.pin() == tally_pin {
                        if state {
                            pin.set_high();
                        } else {
                            pin.set_low();
                        }
                        break;
                    }
                }
            }
        }
    }
}
