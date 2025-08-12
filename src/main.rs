use ghostnet_rs::GhostStream;

fn main() {
    if let Err(_) = ghostnet_rs::gateway::run() {
        let mut stream = GhostStream::connect((0, 0, 0, 0), 7525);
        stream.write("Hello, World".as_bytes());
    }
}
