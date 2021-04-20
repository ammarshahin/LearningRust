use crate::second_app::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn bad_request_handle(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            let mut stream = match listener.accept() {
                Ok(tup) => tup.0,
                Err(e) => panic!("error establish a connection: {}", e),
            };

            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(num) => {
                    println!(
                        "Received a new request of {}byte:\n{}",
                        num,
                        String::from_utf8_lossy(&buffer)
                    );

                    let response = match Request::try_from(&buffer[0..=num]) {
                        Ok(request) => handler.handle_request(&request),
                        Err(e) => handler.bad_request_handle(&e),
                    };

                    if let Err(e) = response.send(&mut stream) {
                        println!("Failed to send the response: {}", e);
                    }
                }
                Err(e) => {
                    println!("Failed to read from connection: {}", e);
                }
            }
        }
    }
}
