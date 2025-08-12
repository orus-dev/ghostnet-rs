use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

use crate::route_tcp;

/// Prints a log message with a timestamp.
fn log(msg: &str) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seconds = now.as_secs();
    let millis = now.subsec_millis();

    // Rough human-readable formatting: seconds since epoch + milliseconds
    println!("[{}.{:03}] {}", seconds, millis, msg);
}

pub fn router() {
    let listener = TcpListener::bind("0.0.0.0:7879").unwrap();
    log("Router started on port 7879");

    for stream in listener.incoming() {
        match stream {
            Ok(mut client_recv) => {
                log(&format!(
                    "Incoming connection from {}",
                    client_recv.peer_addr().unwrap()
                ));

                let mut address = [0; 6];
                let bytes_read = client_recv.read(&mut address).unwrap();
                if bytes_read == 0 {
                    log("Client disconnected before sending address.");
                    continue;
                }
                log(&format!("Received target address: {:?}", address));

                let address = format!(
                    "{}.{}.{}.{}:{}",
                    address[0],
                    address[1],
                    address[2],
                    address[3],
                    u16::from_be_bytes([address[4], address[5]])
                );

                log(&format!("Connecting to routed server at {address}"));

                let mut client_send = client_recv.try_clone().unwrap();
                let mut server_recv = TcpStream::connect(&address).unwrap();
                let mut server_send = server_recv.try_clone().unwrap();

                log(&format!("Connected to routed server at {address}"));

                // Client -> Server
                std::thread::spawn(move || {
                    while !route_tcp(&mut client_recv, &mut server_send) {}
                    log("Client -> Server routing stopped");
                });

                // Server -> Client
                while !route_tcp(&mut client_send, &mut server_recv) {}
                log("Server -> Client routing stopped");
            }
            Err(e) => {
                log(&format!("Connection failed: {}", e));
            }
        }
    }
}

pub fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    let mut connections: Vec<[u8; 6]> = Vec::new();
    log("Gateway started on port 7878");

    std::thread::spawn(router);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                log(&format!(
                    "New client connected from {}",
                    stream.peer_addr().unwrap()
                ));

                let mut address = [0; 6];
                let bytes_read = stream.read(&mut address)?;
                if bytes_read == 0 {
                    log("Client disconnected before sending data.");
                    break;
                }
                log(&format!("Received address: {:?}", address));

                if let Some(con) = connections.last() {
                    log(&format!("Sending previous connection address: {:?}", con));
                    stream.write_all(con)?;
                } else {
                    let default_addr = [0, 0, 0, 0, 0x1E, 0xC7];
                    log(&format!(
                        "No previous connections. Sending default address: {:?}",
                        default_addr
                    ));
                    stream.write_all(&default_addr)?;
                }

                connections.push(address);
                log(&format!(
                    "Stored new connection. Total connections: {}",
                    connections.len()
                ));

                while stream.read(&mut []).unwrap() != 0 {}
                log(&format!(
                    "Client {} disconnected",
                    stream.peer_addr().unwrap()
                ));

                stream.shutdown(Shutdown::Both).unwrap();
                connections.remove(connections.iter().position(|v| v == &address).unwrap());
            }
            Err(e) => {
                log(&format!("Connection failed: {}", e));
            }
        }
    }

    Ok(())
}
