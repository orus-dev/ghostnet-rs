use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

use crate::route_tcp;

pub fn router() {
    let listener = TcpListener::bind("0.0.0.0:7879").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut client_recv) => {
                std::thread::spawn(|| {
                    let mut address = [0; 6];
                    let bytes_read = client_recv.read(&mut address).unwrap();
                    if bytes_read == 0 {
                        return;
                    }

                    let address = format!(
                        "{}.{}.{}.{}:{}",
                        address[0],
                        address[1],
                        address[2],
                        address[3],
                        u16::from_be_bytes([address[4], address[5]])
                    );

                    let mut client_send = client_recv.try_clone().unwrap();
                    let mut server_recv = TcpStream::connect(&address).unwrap();
                    let mut server_send = server_recv.try_clone().unwrap();

                    // Client -> Server
                    std::thread::spawn(move || {
                        while !route_tcp(&mut client_recv, &mut server_send) {}
                        server_send.shutdown(Shutdown::Both).unwrap();
                    });

                    // Server -> Client
                    while !route_tcp(&mut server_recv, &mut client_send) {}
                    client_send.shutdown(Shutdown::Both).unwrap();
                });
            }
            Err(_) => {}
        }
    }
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

                while stream.read(&mut []).unwrap() != 0 {}

                stream.shutdown(Shutdown::Both).unwrap();
                connections.remove(connections.iter().position(|v| v == &address).unwrap());
            }
            Err(_) => {}
        }
    }

    Ok(())
}
