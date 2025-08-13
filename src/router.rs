use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{format_ip_bytes, route_tcp, route_tcp2};

pub fn router() {
    let server = TcpListener::bind("0.0.0.0:443").unwrap();
    let mut clients: Vec<TcpStream> = Vec::new();

    for stream in server.incoming() {
        match stream {
            Ok(mut client_recv) => {
                let mut addr = [0; 6];

                client_recv.read_exact(&mut addr).unwrap();
                clients.push(client_recv.try_clone().unwrap());

                let i = clients.len().wrapping_sub(2);

                if let Some(server_recv) = clients.get_mut(i) {
                    let mut server_send = server_recv.try_clone().unwrap();
                    let mut client_send = client_recv.try_clone().unwrap();

                    server_send
                        .write_all(&[&[1u8], &addr[..]].concat())
                        .unwrap();

                    std::thread::spawn(move || {
                        while !route_tcp2(&mut client_recv, &mut server_send).unwrap() {}
                    });

                    while !route_tcp2(server_recv, &mut client_send).unwrap() {}
                } else {
                    let mut server_recv = TcpStream::connect(format_ip_bytes(addr)).unwrap();
                    let mut server_send = server_recv.try_clone().unwrap();
                    let mut client_send = client_recv.try_clone().unwrap();

                    std::thread::spawn(move || {
                        while !route_tcp(&mut client_recv, &mut server_send).unwrap() {}
                    });

                    while !route_tcp(&mut server_recv, &mut client_send).unwrap() {}
                }
            }
            Err(_) => {}
        }
    }
}
