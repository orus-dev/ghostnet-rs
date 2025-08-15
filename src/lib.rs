use std::io::{Read, Write};

pub mod server;
pub mod tls;

pub fn request(gateway: &str, addr: &str) -> String {
    let (mut conn, mut tcp) = tls::mask_tls(gateway).unwrap();
    let mut tls = rustls::Stream::new(&mut conn, &mut tcp);
    tls.write_all(format!("ROUTE {addr}\nGET / HTTP/1.1").as_bytes())
        .unwrap();
    let mut resp = Vec::new();
    tls.read_to_end(&mut resp).unwrap();
    String::from_utf8_lossy(&resp).to_string()
}
