use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub mod gateway;

pub fn route_tcp(from: &mut TcpStream, to: &mut TcpStream) -> bool {
    let mut data = [0; 2048];

    let bytes_read = from.read(&mut data).unwrap();
    if bytes_read == 0 {
        return true;
    }

    to.write_all(&data[..bytes_read]).unwrap();

    false
}
