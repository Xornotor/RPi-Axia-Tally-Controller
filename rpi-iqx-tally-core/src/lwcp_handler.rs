use crate::json_handler::*;
use std::error::Error;
use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

pub fn open_socket(ip_addr: Ipv4Addr) -> Result<TcpStream, Box<dyn Error>> {
    let socket = SocketAddrV4::new(ip_addr, 4010);
    let mut stream = TcpStream::connect(socket)?;
    stream.set_write_timeout(Some(Duration::from_millis(100)))?;
    stream.set_read_timeout(Some(Duration::from_millis(100)))?;
    stream.set_nonblocking(true)?;
    stream.write(&String::from("LOGIN UNIT").into_bytes())?;
    stream.write(&String::from("SUB GPI#1").into_bytes())?;
    Ok(stream)
}

pub fn read_from_socket(stream: &mut TcpStream) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result: Vec<String> = vec![];
    let mut buf = String::new();
    match stream.read_to_string(&mut buf) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            io::ErrorKind::WouldBlock => {}
            _ => return Err(Box::new(e)),
        },
    };
    for line in buf.lines() {
        result.push(line.to_string());
    }
    Ok(result)
}

pub fn start_connections(tally_cfg: TallyConfig) -> Vec<Receiver<String>> {
    let mut receivers: Vec<Receiver<String>> = vec![];
    for console in tally_cfg.consoles {
        let (tx, rx) = mpsc::channel();
        receivers.push(rx);
        thread::spawn(move || {
            let mut connected: bool = false;
            loop {
                let mut counter: u32 = 0;
                let mut stream = match open_socket(console.ip_addr) {
                    Ok(tcp_stream) => {
                        if !connected {
                            println!("Connected to Axia in address {}", console.ip_addr);
                            connected = true;
                        }
                        tcp_stream
                    }
                    Err(_) => {
                        connected = false;
                        println!("Retrying to connect to {}...", console.ip_addr);
                        continue;
                    }
                };
                loop {
                    let reading = match read_from_socket(&mut stream) {
                        Ok(data) => data,
                        Err(_) => {
                            println!("Failed to read from {}; retrying...", console.ip_addr);
                            continue;
                        }
                    };
                    if reading.len() == 0 {
                        counter += 1;
                    } else {
                        counter = 0;
                    }
                    if counter > 999999 {
                        break;
                    }
                    for line in reading {
                        let _ = tx.send(format!("Console={} {}", console.id_console, line));
                    }
                }
            }
        });
    }
    receivers
}
