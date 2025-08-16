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
        let tcp = TcpStream::connect((self.host.as_str(), 443)).unwrap();
        send_request(tcp, &self.host, &self.to_string())
    }

    pub fn send_routed(&self, router: &str) -> String {
        let mut tcp = TcpStream::connect((router, 80)).unwrap();
        tcp.write_all(format!("ROUTE {}:443", self.host).as_bytes())
            .unwrap();
        tcp.read(&mut [0; 32]).unwrap();
        send_request(tcp, &self.host, &self.to_string())
    }

    pub fn send_routed_secure(&self, router: &str) -> String {
        let tcp = TcpStream::connect((router, 443)).unwrap();
        send_request(
            tcp.try_clone().unwrap(),
            &self.host,
            &format!("ROUTE {}:443", self.host),
        );
        send_request(tcp, &self.host, &self.to_string())
    }

    pub fn to_string(&self) -> String {
        format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nUser-Agent: rustls/0.23\r\n\r\n",
            self.path, self.host
        )
    }
}

pub fn send_request(mut stream: TcpStream, host: &str, send: &str) -> String {
    let mut conn = tls::tls13_handshake(&host, &mut stream).unwrap();
    let mut tls = rustls::Stream::new(&mut conn, &mut stream);
    tls.write_all(send.as_bytes()).unwrap();
    tls.flush().unwrap();
    let mut resp = Vec::new();
    tls.read_to_end(&mut resp).unwrap();
    String::from_utf8(resp).unwrap()
}
