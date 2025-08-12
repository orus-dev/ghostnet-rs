use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub mod gateway;

pub struct GhostStream(TcpStream);

impl GhostStream {
    pub fn connect(ip: (u8, u8, u8, u8), port: u16) -> std::io::Result<Self> {
        let addr = get_addr()?;
        let mut stream = TcpStream::connect(addr)?;
        let bep = port.to_be_bytes();
        stream.write_all(&[ip.0, ip.1, ip.2, ip.3, bep[0], bep[1]])?;

        Ok(Self(stream))
    }

    pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.0.write_all(data)
    }

    pub fn read(&mut self, data: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(data)
    }
}

pub fn get_addr() -> std::io::Result<String> {
    let mut stream = TcpStream::connect("0.0.0.0:7878")?;

    stream.write_all(&[0, 0, 0, 0, 0x1E, 0xC7])?;

    let mut address = [0; 6];

    stream.read(&mut address)?;

    Ok(format!(
        "{}.{}.{}.{}:{}",
        address[0],
        address[1],
        address[2],
        address[3],
        u16::from_be_bytes([address[4], address[5]])
    ))
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
