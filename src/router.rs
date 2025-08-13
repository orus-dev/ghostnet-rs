use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use crate::{format_ip_bytes, route_tcp, route_tcp2};

pub fn router() {
    let server = TcpListener::bind("0.0.0.0:443").unwrap();
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in server.incoming() {
        match stream {
            Ok(mut client_recv) => {
                let clients = clients.clone();
                std::thread::spawn(move || {
                    let mut addr = [0; 6];

                    client_recv.read_exact(&mut addr).unwrap();
                    clients
                        .lock()
                        .unwrap()
                        .push(client_recv.try_clone().unwrap());

                    let i = clients.lock().unwrap().len().wrapping_sub(2);

                    let mut using_router = false;

                    let mut server_recv = if let Some(server_recv) = clients.lock().unwrap().get(i)
                    {
                        server_recv.try_clone().unwrap()
                    } else {
                        using_router = true;
                        TcpStream::connect(format_ip_bytes(addr)).unwrap()
                    };

                    let mut server_send = server_recv.try_clone().unwrap();
                    let mut client_send = client_recv.try_clone().unwrap();

                    if using_router {
                        std::thread::spawn(move || {
                            while !route_tcp(&mut client_recv, &mut server_send).unwrap() {}
                        });

                        while !route_tcp(&mut server_recv, &mut client_send).unwrap() {}
                    } else {
                        server_send
                            .write_all(&[&[1u8], &addr[..]].concat())
                            .unwrap();

                        std::thread::spawn(move || {
                            while !route_tcp2(&mut client_recv, &mut server_send).unwrap() {}
                        });

                        while !route_tcp2(&mut server_recv, &mut client_send).unwrap() {}
                    }
                });
            }
            Err(_) => {}
        }
    }
}
