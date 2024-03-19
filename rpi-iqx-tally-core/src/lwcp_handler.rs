use std::error::Error;
use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::time::Duration;

pub fn open_socket(ip_addr: Ipv4Addr) -> Result<TcpStream, Box<dyn Error>> {
    let socket = SocketAddrV4::new(ip_addr, 4010);
    let mut stream = TcpStream::connect(socket)?;
    stream.set_write_timeout(Some(Duration::from_millis(100)))?;
    stream.set_read_timeout(Some(Duration::from_millis(100)))?;
    stream.set_nonblocking(true)?;
    stream.write(&String::from("LOGIN UNIT").into_bytes())?;
    stream.write(&String::from("SUB GPI#1").into_bytes())?;
    println!("Connected to Axia in address {}", ip_addr);
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
