mod json_handler;
mod lwcp_handler;
use std::net::TcpStream;
//use std::thread;
use crate::json_handler::*;
use crate::lwcp_handler::*;

fn main() {
    let tally_cfg = init_tally_config().expect("Error accessing tally config file");
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
}
