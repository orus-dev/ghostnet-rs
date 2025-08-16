use ghostnet_rs::Request;

fn main() {
    ghostnet_rs::server::run();
    // println!("{}", Request::new("example.com").send_routed_secure("localhost"));
    // println!("{}", Request::new("example.com").send_routed("localhost"));
    // println!("{}", Request::new("example.com").send());
}
