use ghostnet_rs::GhostStream;

fn main() {
    let mut stream = GhostStream::connect((0, 0, 0, 0), 7525).unwrap();
    stream.write("Hello, World".as_bytes()).unwrap();
    let mut data = [0; 12];
    let data_len = stream.read(&mut data).unwrap();
    if data_len == 0 {
        return;
    }

    println!("From server: {}", String::from_utf8(data.to_vec()).unwrap());
}
