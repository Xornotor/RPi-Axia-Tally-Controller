mod json_handler;
mod lwcp_handler;
mod gpio_driver;

use crate::json_handler::*;
use crate::lwcp_handler::*;
use crate::gpio_driver::*;

use rppal::gpio::Gpio;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::{self, Receiver};

fn main() {
    // Gather Tally Config File
    let tally_cfg = init_tally_config().expect("Error accessing tally config file");

    // GPIO Initializing
    let gpio = Gpio::new().expect("Failed while configuring GPIO Handler");
    let mut tally_pins = init_gpio(&gpio, &tally_cfg).expect("Failed to initialize GPIO");
    reset_all_gpio(&mut tally_pins);
    
    let mut receivers: Vec<Receiver<String>> = vec![];
    for console in tally_cfg.consoles {
        let (tx, rx) = mpsc::channel();
        receivers.push(rx);
        thread::spawn(move || {
            let mut stream = open_socket(console.ip_addr).unwrap();
            loop {
                let reading = read_from_socket(&mut stream).unwrap();
                for line in reading {
                    tx.send(format!("Console{} {}", console.id_console, line));
                }
            }
        });
    }

    loop {
        for receiver in &receivers{
            match receiver.try_recv() {
                Ok(msg) => println!("{msg}"),
                Err(_) => {},
            }
            
        }
    }

    /*
    // Open socket connections with Axia Consoles
    let mut streams: Vec<TcpStream> = vec![];
    for console in tally_cfg.consoles {
        let mut stream = open_socket(console.ip_addr).unwrap();
        streams.push(stream);
    }

    loop {
        for mut stream in &mut streams {
            let reading = read_from_socket(&mut stream).unwrap();
            for line in reading {
                println!("{line}");
            }
        }
    }
    */
}
