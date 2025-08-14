use ghostnet_rs::request;

fn main() {
    let result = request("localhost", "github.com");

    println!("Result: {result}");
}
