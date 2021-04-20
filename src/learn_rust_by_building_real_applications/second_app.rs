mod http;
mod request_handler;
mod server;

use request_handler::RequestHandler;
use server::Server;

pub fn run() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(RequestHandler);
}
