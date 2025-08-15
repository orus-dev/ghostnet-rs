use rustls::ClientConfig;
use rustls::client::ClientConnection;
use std::{net::TcpStream, sync::Arc};

pub fn mask_tls<'a>(
    addr: &str,
) -> Result<(ClientConnection, TcpStream), Box<dyn std::error::Error>> {
    let certs = rustls_native_certs::load_native_certs()
        .expect("could not load platform certificate store");
    let mut root_store = rustls::RootCertStore::empty();
    for cert in certs {
        root_store.add(cert).unwrap();
    }

    // Build TLS client config
    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let arc_cfg = Arc::new(config);
    let conn = ClientConnection::new(
        arc_cfg,
        addr.split_once(":").unwrap().0.to_string().try_into()?,
    )?;
    let tcp = TcpStream::connect(addr)?;
    Ok((conn, tcp))
}
