mod http;
mod server;
use http::Method;
use http::Request;
use server::Server;

pub fn run() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
