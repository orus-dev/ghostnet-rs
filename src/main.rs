use ghostnet_rs::GhostClient;

fn main() {
    // ghostnet_rs::router::router();
    let mut con = GhostClient::connect("0.0.0.0:443", (0, 0, 0, 0), 7525).unwrap();
    con.send("Hello from client!".as_bytes()).unwrap();
    let response = con.read_bytes(1024).unwrap();
    println!("From server: {}", String::from_utf8(response).unwrap());
    println!("From server 2: {:?}", con.read_bytes(1024).unwrap());
}
