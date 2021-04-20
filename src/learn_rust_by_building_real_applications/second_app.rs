mod http;
mod request_handler;
mod server;
use request_handler::RequestHandler;
use server::Server;
use std::env;

pub fn run() {
    let default_path = format!(
        "{}/src/learn_rust_by_building_real_applications/public",
        env!("CARGO_MANIFEST_DIR") // this macro returns at compile time the path of the directory that contains cargo.toml
    );
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(RequestHandler::new(public_path));
}
