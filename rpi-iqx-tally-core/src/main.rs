mod event_parser;
mod gpio_driver;
mod json_handler;
mod lwcp_handler;

use crate::gpio_driver::*;
use crate::json_handler::*;
use crate::lwcp_handler::*;

use rppal::gpio::Gpio;

fn main() {
    // Gather Tally Config File
    let tally_cfg = init_tally_config().expect("Error accessing tally config file");

    // GPIO Initializing
    let gpio = Gpio::new().expect("Failed while configuring GPIO Handler");
    let mut tally_pins = init_gpio(&gpio, &tally_cfg).expect("Failed to initialize GPIO");
    reset_all_gpio(&mut tally_pins);

    let receivers = start_connections(tally_cfg.clone());

    // Continuous message receiving and GPIO management
    loop {
        decode_receivers_to_gpio(&receivers, &tally_cfg, &mut tally_pins);
    }
}
