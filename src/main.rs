use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("ipv4.icanhazip.com:80")?;

    // Send raw HTTP request
    let request = "GET / HTTP/1.1\r\n\
                   Host: ipv4.icanhazip.com\r\n\
                   Connection: close\r\n\
                   \r\n";
    stream.write_all(request.as_bytes())?;

    // Read full response
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Extract body after headers
    if let Some(pos) = response.find("\r\n\r\n") {
        let body = response[pos + 4..].trim();

        // Parse into tuple (u8, u8, u8, u8)
        let octets: Vec<u8> = body
            .split('.')
            .filter_map(|s| s.parse::<u8>().ok())
            .collect();

        if octets.len() == 4 {
            let ip_tuple = (octets[0], octets[1], octets[2], octets[3]);
            ghostnet_rs::gateway::run(ip_tuple)?;
        } else {
            eprintln!("Invalid IPv4 address format: {}", body);
        }
    } else {
        eprintln!("Invalid HTTP response");
    }

    Ok(())
}
