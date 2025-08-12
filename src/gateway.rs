use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

use crate::route_tcp;

pub fn router() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7879")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut client_recv) => {
                std::thread::spawn(|| -> std::io::Result<()> {
                    let mut address = [0; 6];
                    let bytes_read = client_recv.read(&mut address)?;
                    if bytes_read == 0 {
                        return Ok(());
                    }

                    let address = format!(
                        "{}.{}.{}.{}:{}",
                        address[0],
                        address[1],
                        address[2],
                        address[3],
                        u16::from_be_bytes([address[4], address[5]])
                    );

                    let mut client_send = client_recv.try_clone()?;
                    let mut server_recv = TcpStream::connect(&address)?;
                    let mut server_send = server_recv.try_clone()?;

                    // Client -> Server
                    std::thread::spawn(move || -> std::io::Result<()> {
                        while !route_tcp(&mut client_recv, &mut server_send)? {}
                        server_send.shutdown(Shutdown::Both)?;
                        Ok(())
                    });

                    // Server -> Client
                    while !route_tcp(&mut server_recv, &mut client_send)? {}
                    client_send.shutdown(Shutdown::Both)?;

                    Ok(())
                });
            }
            Err(_) => {}
        }
    }

    Ok(())
}

pub fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    let mut connections: Vec<[u8; 6]> = Vec::new();

    std::thread::spawn(router);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut address = [0; 6];
                let bytes_read = stream.read(&mut address)?;
                if bytes_read == 0 {
                    break;
                }

                if let Some(con) = connections.last() {
                    stream.write_all(con)?;
                } else {
                    let default_addr = [0, 0, 0, 0, 0x1E, 0xC7];
                    stream.write_all(&default_addr)?;
                }

                connections.push(address);

                while stream.read(&mut [])? != 0 {}

                stream.shutdown(Shutdown::Both)?;
                if let Some(v) = connections.iter().position(|v| v == &address) {
                    connections.remove(v);
                }
            }
            Err(_) => {}
        }
    }

    Ok(())
}
