use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

use crate::route_tcp;

pub fn router() {
    let listener = TcpListener::bind("0.0.0.0:7879").unwrap();
    println!("Router started on port 7879");

    for stream in listener.incoming() {
        match stream {
            Ok(mut client_recv) => {
                std::thread::spawn(|| {
                    println!(
                        "Incoming connection from {}",
                        client_recv.peer_addr().unwrap()
                    );

                    let mut address = [0; 6];
                    let bytes_read = client_recv.read(&mut address).unwrap();
                    if bytes_read == 0 {
                        println!("Client disconnected before sending address.");
                    }
                    println!("Received target address: {:?}", address);

                    let address = format!(
                        "{}.{}.{}.{}:{}",
                        address[0],
                        address[1],
                        address[2],
                        address[3],
                        u16::from_be_bytes([address[4], address[5]])
                    );

                    println!("Connecting to routed server at {address}");

                    let mut client_send = client_recv.try_clone().unwrap();
                    let mut server_recv = TcpStream::connect(&address).unwrap();
                    let mut server_send = server_recv.try_clone().unwrap();

                    println!("Connected to routed server at {address}");

                    // Client -> Server
                    std::thread::spawn(move || {
                        while !route_tcp(&mut client_recv, &mut server_send) {}
                        server_send.shutdown(Shutdown::Both).unwrap();
                        println!("Client -> Server routing stopped");
                    });

                    // Server -> Client
                    while !route_tcp(&mut server_recv, &mut client_send) {}
                    client_send.shutdown(Shutdown::Both).unwrap();
                    println!("Server -> Client routing stopped");
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

pub fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    let mut connections: Vec<[u8; 6]> = Vec::new();
    println!("Gateway started on port 7878");

    std::thread::spawn(router);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New client connected from {}", stream.peer_addr().unwrap());

                let mut address = [0; 6];
                let bytes_read = stream.read(&mut address)?;
                if bytes_read == 0 {
                    println!("Client disconnected before sending data.");
                    break;
                }
                println!("Received address: {:?}", address);

                if let Some(con) = connections.last() {
                    println!("Sending previous connection address: {:?}", con);
                    stream.write_all(con)?;
                } else {
                    let default_addr = [0, 0, 0, 0, 0x1E, 0xC7];
                    println!(
                        "No previous connections. Sending default address: {:?}",
                        default_addr
                    );
                    stream.write_all(&default_addr)?;
                }

                connections.push(address);
                println!(
                    "Stored new connection. Total connections: {}",
                    connections.len()
                );

                while stream.read(&mut []).unwrap() != 0 {}
                println!("Client {} disconnected", stream.peer_addr().unwrap());

                stream.shutdown(Shutdown::Both).unwrap();
                connections.remove(connections.iter().position(|v| v == &address).unwrap());
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
