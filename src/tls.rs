use std::net::TcpStream;
use std::sync::Arc;

use rustls::client::ClientConfig;
use rustls::pki_types::ServerName;
use rustls::{ClientConnection, RootCertStore};

pub fn root_store() -> RootCertStore {
    // Prefer system roots (works on most OSes). If that fails, fall back to webpki-roots.
    let mut store = RootCertStore::empty();

    // Try load native (ignore per-cert errors, just skip bad ones)
    for cert in rustls_native_certs::load_native_certs().certs {
        let _ = store.add(cert);
    }

    if store.is_empty() {
        // Fallback: baked-in Mozilla roots via webpki-roots
        store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    }

    store
}

pub fn tls13_config() -> Arc<ClientConfig> {
    let mut cfg = ClientConfig::builder()
        .with_root_certificates(root_store())
        .with_no_client_auth();

    // ALPN (optional but typical)
    cfg.alpn_protocols = vec![b"http/1.1".to_vec()];

    // Pin to TLS 1.3 only
    // cfg.versions = vec![rustls::version::TLS13];

    Arc::new(cfg)
}

pub fn tls13_handshake(
    host: &str,
    tcp: &mut TcpStream,
) -> Result<ClientConnection, Box<dyn std::error::Error>> {
    // SNI + config
    let server_name = ServerName::try_from(host.to_string())?;
    let mut conn = ClientConnection::new(tls13_config(), server_name)?;

    // Drive the handshake to completion (blocking)
    while conn.is_handshaking() {
        // complete_io performs any pending write(s) and then tries to read.
        // It returns Ok((nw, nr)) when some I/O happened; errors propagate.
        let _ = conn.complete_io(tcp)?;
    }

    Ok(conn)
}

// fn main() -> anyhow::Result<()> {
//     let host = "example.com";

//     // 1) TLS 1.3 handshake over a TcpStream
//     let (mut conn, mut tcp) = tls13_handshake(host, 443)?;

//     // 2) After handshake, you can wrap into a rustls::Stream to do Read/Write of app data
//     let mut tls = rustls::Stream::new(&mut conn, &mut tcp);

//     // Simple HTTP/1.1 GET (for demonstration)
//     write!(
//         tls,
//         "GET / HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\nUser-Agent: rustls/0.23\r\n\r\n"
//     )?;
//     tls.flush()?;

//     // Read response
//     let mut resp = Vec::new();
//     tls.read_to_end(&mut resp)?;
//     println!("{}", String::from_utf8_lossy(&resp));

//     Ok(())
// }
