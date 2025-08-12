use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::route_tcp;

pub fn router() {
    let listener = TcpListener::bind("0.0.0.0:7879").unwrap();
    println!("Router started on port 7879");

    for stream in listener.incoming() {
        match stream {
            Ok(mut client_recv) => {
                let mut address = [0; 6];
                let bytes_read = client_recv.read(&mut address).unwrap();
                if bytes_read == 0 {
                    continue;
                }

                let mut client_send = client_recv.try_clone().unwrap();
                let mut server_recv = TcpStream::connect("127.0.0.1:7878").unwrap();
                let mut server_send = server_recv.try_clone().unwrap();

                std::thread::spawn(move || while !route_tcp(&mut client_recv, &mut server_send) {});

                while !route_tcp(&mut client_send, &mut server_recv) {}
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
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
                let mut address = [0; 6];
                let bytes_read = stream.read(&mut address)?;
                if bytes_read == 0 {
                    break;
                }

                println!("Received: {:?}", &address);

                if let Some(con) = connections.last() {
                    stream.write_all(con)?;
                } else {
                    stream.write_all(&[127, 0, 0, 1, 0x1E, 0xBF])?;
                }

                connections.push(address);

                while stream.read(&mut []).unwrap() != 0 {}

                stream.shutdown(std::net::Shutdown::Both).unwrap();
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
