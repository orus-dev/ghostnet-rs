use ghostnet_rs::Request;

fn main() {
    let req = Request::new("example.com");
    println!("{}", req.send());
}
