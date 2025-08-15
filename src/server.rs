use std::io::{Read, Write};
use std::net::TcpListener;
use std::str::FromStr;

use reqwest::Version;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn run() {
    let listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        std::env::var("PORT").unwrap_or_else(|_| "3000".into())
    ))
    .unwrap();

    loop {
        let (mut stream, _) = listener.accept().unwrap();

        let mut buf = [0; 1024];
        if let Ok(len) = stream.read(&mut buf) {
            let buf_str = String::from_utf8_lossy(&buf[..len]).to_string();
            if buf_str.starts_with("ROUTE") {
                let addr = buf_str
                    .split_once("\n")
                    .unwrap()
                    .0
                    .split_once(" ")
                    .unwrap()
                    .1;
                let http = buf_str.split_once("\n").unwrap().1;
                stream
                    .write_all(send_request(addr, http).as_bytes())
                    .unwrap();
                stream.flush().unwrap();
            } else {
                stream
                    .write_all(send_request("https://wikipedia.org", "GET / HTTP/1.1").as_bytes())
                    .unwrap();
                stream.flush().unwrap();
            }
        }
    }
}

fn send_request(url: &str, request: &str) -> String {
    let client = Client::new();
    let mut lines = request.lines();

    let request_line = lines.next().unwrap();
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    // Ignore version for now

    let mut headers = Vec::new();
    for line in lines.clone() {
        if line.is_empty() {
            break; // end of headers
        }
        if let Some((key, value)) = line.split_once(':') {
            headers.push((
                HeaderName::from_str(key.trim()).unwrap(),
                HeaderValue::from_str(value.trim()).unwrap(),
            ));
        }
    }

    let body: String = lines.collect::<Vec<_>>().join("\n");

    let builder = match method {
        "GET" => client.get(url),
        "POST" => client.post(url).body(body.clone()),
        "PUT" => client.put(url).body(body.clone()),
        "DELETE" => client.delete(url),
        _ => client.get(url),
    }
    .headers(HeaderMap::from_iter(headers.into_iter()));

    let resp = builder.send().unwrap();

    let mut http_string = String::new();

    http_string.push_str(&format!(
        "HTTP/{} {} {}\r\n",
        match resp.version() {
            Version::HTTP_09 => "0.9",
            Version::HTTP_10 => "1.0",
            Version::HTTP_11 => "1.1",
            Version::HTTP_2 => "2.0",
            Version::HTTP_3 => "3.0",
            _ => "2.0",
        },
        resp.status().as_u16(),
        resp.status().canonical_reason().unwrap_or("")
    ));

    for (key, value) in resp.headers().iter() {
        http_string.push_str(&format!("{}: {}\r\n", key, value.to_str().unwrap()));
    }

    http_string.push_str("\r\n");

    let body = resp.text().unwrap();
    http_string.push_str(&body);

    http_string
}
