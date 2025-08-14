use std::io::{Read, Write};

use ghostnet_rs::tls;
use rustls::Stream;

fn main() {
    let (mut conn, mut tcp) = tls::mask_tls("github.com").unwrap();
    let mut tls = Stream::new(&mut conn, &mut tcp);

    tls.write_all("GET / HTTP/1.0\r\nHost: github.com\r\n\r\n".as_bytes())
        .unwrap();

    let mut resp = Vec::new();
    tls.read_to_end(&mut resp).unwrap();
    println!("{}", String::from_utf8_lossy(&resp));
}
