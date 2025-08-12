use ghostnet_rs::GhostStream;

fn main() {
    if let Err(_) = ghostnet_rs::gateway::run() {
        let mut stream = GhostStream::connect((0, 0, 0, 0), 7525);
        stream.write("Hello, World".as_bytes());
        let mut data = [0; 12];
        let data_len = stream.read(&mut data);
        if data_len == 0 {
            return;
        }

        println!("From server: {}", String::from_utf8(data.to_vec()).unwrap());
    }
}
