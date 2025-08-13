use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub mod router;

pub type IpAddr = (u8, u8, u8, u8);

pub struct GhostClient(TcpStream);

impl GhostClient {
    pub fn connect(router: &str, host: IpAddr, port: u16) -> std::io::Result<Self> {
        let mut stream = TcpStream::connect(router)?;

        let port = port.to_be_bytes();

        stream.write_all(&[host.0, host.1, host.2, host.3, port[0], port[1]])?;

        Ok(GhostClient(stream))
    }
}

impl GhostClient {
    pub fn send(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.0.write_all(buf)
    }

    pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }

    pub fn read_bytes(&mut self, size: usize) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        let n = self.read(&mut buf)?;
        buf.truncate(n);
        Ok(buf)
    }
}

pub fn format_ip_bytes(buf: [u8; 6]) -> String {
    format!(
        "{}.{}.{}.{}:{}",
        buf[0],
        buf[1],
        buf[2],
        buf[3],
        u16::from_be_bytes([buf[4], buf[5]])
    )
}

pub fn route_tcp(from: &mut TcpStream, to: &mut TcpStream) -> std::io::Result<bool> {
    let mut data = [0; 2048];

    let bytes_read = from.read(&mut data)?;
    if bytes_read == 0 {
        return Ok(true);
    }

    to.write_all(&data[..bytes_read])?;

    Ok(false)
}

pub fn route_tcp2(from: &mut TcpStream, to: &mut TcpStream) -> std::io::Result<bool> {
    let mut data = [0; 2048];

    let bytes_read = from.read(&mut data)?;
    if bytes_read == 0 {
        return Ok(true);
    }

    to.write_all(&[&[0u8], &data[..bytes_read]].concat())?;

    Ok(false)
}
