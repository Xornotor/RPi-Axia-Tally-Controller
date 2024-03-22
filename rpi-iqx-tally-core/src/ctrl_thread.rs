use crate::gpio_driver::*;
use crate::json_handler::*;
use crate::lwcp_handler::*;

use std::thread::{self, JoinHandle};
use rppal::gpio::Gpio;
use std::sync::mpsc::{self, Sender};

pub fn start_ctrl_thread() -> (Box<JoinHandle<()>>, Sender<String>) {

	let (tx_ctrl_kill, rx_ctrl_kill) = mpsc::channel();
	let ctrl_thread = thread::spawn(move || {
		// Gather Tally Config File
		let tally_cfg = init_tally_config().expect("Error accessing tally config file");

		// GPIO Initializing
		let gpio = Gpio::new().expect("Failed while configuring GPIO Handler");
		let mut tally_pins = init_gpio(&gpio, &tally_cfg).expect("Failed to init GPIO");
		reset_all_gpio(&mut tally_pins);

		let (senders, receivers, mut handlers) = start_connections(tally_cfg.clone());
		
		// Continuous message receiving and GPIO management
		loop {
			let kill_msg = match rx_ctrl_kill.try_recv() {
				Ok(recv_msg) => recv_msg,
				Err(_) => String::new(),
			};
			if kill_msg == String::from("KILL"){
				println!("Killing controller thread...");
				for sender in &senders{
					let _ = sender.send(String::from("KILL"));
				}
				'handler_loop: for _ in 0..handlers.len() {
					let handler = handlers.pop();
					let _join = match handler {
						Some(h) => h.join(),
						None => { break 'handler_loop; },
					};
				}
				break;
			}
			decode_receivers_to_gpio(&receivers, &tally_cfg, &mut tally_pins);
		}
	});
	
	(Box::new(ctrl_thread), tx_ctrl_kill)
}
