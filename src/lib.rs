use std::io::{Read, Write};

use rustls::Stream;

pub mod tls;

pub fn request(gateway: &str, addr: &str) -> String {
    // Send a request to the gateway and provide the IP address and request
    let (mut conn, mut tcp) = tls::mask_tls(gateway).unwrap();
    let mut tls = Stream::new(&mut conn, &mut tcp);
    tls.write_all(format!("ROUTE {addr}").as_bytes()).unwrap();
    let mut resp = Vec::new();
    tls.read_to_end(&mut resp).unwrap();
    String::from_utf8_lossy(&resp).to_string()
}
