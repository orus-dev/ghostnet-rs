use std::error::Error as StdError;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;

use rustls::Stream;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

pub fn run() -> Result<(), Box<dyn StdError>> {
    let cert_file = "cert.pem";
    let private_key_file = "key.pem";

    let certs = CertificateDer::pem_file_iter(cert_file)
        .unwrap()
        .map(|cert| cert.unwrap())
        .collect();

    let private_key = PrivateKeyDer::from_pem_file(private_key_file).unwrap();

    let config = Arc::new(
        rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, private_key)?,
    );

    let listener = TcpListener::bind(format!("0.0.0.0:{}", 443)).unwrap();

    loop {
        let (mut tcp_stream, _) = listener.accept()?;
        let mut conn = rustls::ServerConnection::new(config.clone())?;
        let mut tls_stream = rustls::Stream::new(&mut conn, &mut tcp_stream);

        let mut buf = [0; 1024];
        if let Ok(len) = tls_stream.read(&mut buf) {
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
                tls_stream
                    .write_all(&send_request(addr, http.as_bytes()))
                    .unwrap();
                tls_stream.flush()?;
            } else {
                tls_stream
                    .write_all(&send_request("github.com", buf_str.as_bytes()))
                    .unwrap();
                tls_stream.flush()?;
            }
        }
    }
}

fn send_request(addr: &str, buf: &[u8]) -> Vec<u8> {
    let (mut conn, mut tcp) = crate::tls::mask_tls(addr).unwrap();
    let mut tls = Stream::new(&mut conn, &mut tcp);
    tls.write_all(buf).unwrap();
    let mut resp = Vec::new();
    tls.read_to_end(&mut resp).unwrap();
    resp
}
