mod server;
mod http;
use server::Server;
use http::{Request, Method};


fn main() {
    println!("server running 😡");
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
