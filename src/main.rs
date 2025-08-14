// Cargo.toml dependencies:
// rustls = "0.31"
// rustls-native-certs = "0.8"

use rustls::ClientConfig;
use rustls::Stream;
use rustls::client::ClientConnection;
use rustls::server::CertificateType;
use rustls_native_certs;
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load native certificates
    let certs = rustls_native_certs::load_native_certs()
        .expect("could not load platform certificate store");
    let mut root_store = rustls::RootCertStore::empty();
    for cert in certs {
        root_store.add(cert).unwrap();
    }

    // Build TLS client config
    let config = ClientConfig::builder()
        // .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let arc_cfg = Arc::new(config);
    let server_name = "example.com".try_into()?;
    let mut conn = ClientConnection::new(arc_cfg, server_name)?;
    let mut tcp = TcpStream::connect("example.com:443")?;
    let mut tls = Stream::new(&mut conn, &mut tcp);

    tls.write_all(b"GET / HTTP/1.0\r\nHost: example.com\r\n\r\n")?;
    let mut resp = Vec::new();
    tls.read_to_end(&mut resp)?;
    println!("{}", String::from_utf8_lossy(&resp));

    Ok(())
}
