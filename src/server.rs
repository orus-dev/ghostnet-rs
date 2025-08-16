use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn run() {
    let server = TcpListener::bind(format!(
        "0.0.0.0:{}",
        std::env::var("PORT").unwrap_or("80".to_string())
    ))
    .unwrap();

    loop {
        for stream in server.incoming() {
            match stream {
                Ok(mut client) => {
                    println!("Client connected");
                    let mut buf = [0; 1024];

                    let buf_len = client.read(&mut buf).unwrap();

                    let req = String::from_utf8_lossy(&buf[..buf_len]);
                    let req = req.lines().collect::<Vec<_>>();

                    if req.len() == 0 {
                        continue;
                    }

                    if req[0].starts_with("ROUTE") {
                        println!("Route mode");
                        let addr = req[0].split_once(' ').unwrap().1;
                        println!("Connecting to {addr}");
                        let mut target = TcpStream::connect(addr).unwrap();
                        let mut target_t = target.try_clone().unwrap();
                        let mut client_t = client.try_clone().unwrap();
                        println!("Connected to {addr}");
                        client.write_all("CONN EST".as_bytes()).unwrap();

                        std::thread::spawn(move || {
                            loop {
                                let mut buf = [0; 2048];
                                let buf_len = client_t.read(&mut buf).unwrap();
                                if buf_len == 0 {
                                    target_t.shutdown(std::net::Shutdown::Both).unwrap();
                                    break;
                                }
                                println!("Client -> Target {:?}", &buf[..buf_len]);
                                target_t.write_all(&buf[..buf_len]).unwrap();
                            }
                        });

                        loop {
                            let mut buf = [0; 2048];
                            let buf_len = target.read(&mut buf).unwrap();
                            if buf_len == 0 {
                                client.shutdown(std::net::Shutdown::Both).unwrap();
                                break;
                            }
                            println!("Target -> Client {:?}", &buf[..buf_len]);
                            client.write_all(&buf[..buf_len]).unwrap();
                        }
                    }
                }

                Err(_) => {}
            }
        }
    }
}
