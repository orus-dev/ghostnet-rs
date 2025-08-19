use std::{
    io::{Read, Write},
    net::TcpStream,
};

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Patch,
    Connect,
    Trace,
}

pub mod server;
pub mod tls;

#[derive(Debug)]
pub struct Request {
    host: String,
    path: String,
    method: Method,
}

impl Request {
    pub fn new(url: &str) -> Self {
        Self {
            host: url.to_string(),
            path: String::from("/"),
            method: Method::Get,
        }
    }

    pub fn from_str(host: &str, req: &str) -> Self {
        let first_line = req.lines().next().unwrap_or_default();
        let mut parts = first_line.split_whitespace();
        let method = parts.next().unwrap_or_default();
        let path = parts.next().unwrap_or("/");

        Self {
            host: host.to_string(),
            path: path.to_string(),
            method: match method {
                "GET" => Method::Get,
                "POST" => Method::Post,
                "PUT" => Method::Put,
                "DELETE" => Method::Delete,
                "HEAD" => Method::Head,
                "OPTIONS" => Method::Options,
                "PATCH" => Method::Patch,
                "CONNECT" => Method::Connect,
                "TRACE" => Method::Trace,
                _ => Method::Get,
            },
        }
    }
}

impl Request {
    pub fn send_bytes(&self) -> Vec<u8> {
        let tcp = TcpStream::connect((self.host.as_str(), 443)).unwrap();
        send_request(tcp, &self.host, &self.to_string())
    }

    pub fn send(&self) -> String {
        let tcp = TcpStream::connect((self.host.as_str(), 443)).unwrap();
        String::from_utf8(send_request(tcp, &self.host, &self.to_string())).unwrap()
    }

    pub fn send_routed(&self, router: &str) -> String {
        let mut tcp = TcpStream::connect(router).unwrap();
        tcp.write_all(format!("ROUTE {}:443", self.host).as_bytes())
            .unwrap();
        tcp.read(&mut [0; 32]).unwrap();
        String::from_utf8(send_request(tcp, &self.host, &self.to_string())).unwrap()
    }

    pub fn send_routed_secure(&self, router: &str) -> String {
        let tcp = TcpStream::connect((router, 443)).unwrap();
        send_request(
            tcp.try_clone().unwrap(),
            &router,
            &format!("ROUTE {}:443", self.host),
        );
        String::from_utf8(send_request(tcp, &self.host, &self.to_string())).unwrap()
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nUser-Agent: rustls/0.23\r\n\r\n",
            self.method, self.path, self.host
        )
    }
}

pub fn send_request(mut stream: TcpStream, host: &str, send: &str) -> Vec<u8> {
    let mut conn = tls::tls13_handshake(&host, &mut stream).unwrap();
    let mut tls = rustls::Stream::new(&mut conn, &mut stream);
    tls.write_all(send.as_bytes()).unwrap();
    tls.flush().unwrap();
    let mut resp = Vec::new();
    tls.read_to_end(&mut resp).unwrap();
    resp
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Get => "GET",
                Self::Post => "POST",
                Self::Put => "PUT",
                Self::Delete => "DELETE",
                Self::Head => "HEAD",
                Self::Options => "OPTIONS",
                Self::Patch => "PATCH",
                Self::Connect => "CONNECT",
                Self::Trace => "TRACE",
            }
        )
    }
}
