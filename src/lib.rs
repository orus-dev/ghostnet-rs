use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub mod server;
pub mod tls;

pub struct Request {
    host: String,
    path: String,
}

impl Request {
    pub fn new(url: &str) -> Self {
        Self {
            host: url.to_string(),
            path: String::from("/"),
        }
    }
}

impl Request {
    pub fn send(&self) -> String {
        let mut tcp = TcpStream::connect((self.host.as_str(), 443)).unwrap();
        let mut conn = tls::tls13_handshake(&self.host, &mut tcp).unwrap();
        let mut tls = rustls::Stream::new(&mut conn, &mut tcp);
        write!(
            tls,
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nUser-Agent: rustls/0.23\r\n\r\n",
            self.path, self.host
        )
        .unwrap();
        tls.flush().unwrap();
        let mut resp = Vec::new();
        tls.read_to_end(&mut resp).unwrap();
        String::from_utf8(resp).unwrap()
    }
}
