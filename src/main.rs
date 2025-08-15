#[tokio::main]
async fn main() {
    ghostnet_rs::server::run().await.unwrap();
    // println!(
    // "{}",
    // ghostnet_rs::request("ghostnet-rs.onrender.com:443", "https://wikipedia.org")
    // );
}
