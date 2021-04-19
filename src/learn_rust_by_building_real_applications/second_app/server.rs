use crate::second_app::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
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

                    let result = match Request::try_from(&buffer[0..=num]) {
                        Ok(request) => {
                            let response = Response::new(
                                StatusCode::Ok,
                                Some("<h1>IT Works!!!!</h1>".to_string()),
                            );
                            dbg!(request);
                            response.send(&mut stream)
                        }
                        Err(e) => {
                            println!("Failed to parse request: {}", e);
                            let response = Response::new(StatusCode::BadRequest, None);
                            response.send(&mut stream)
                        }
                    };

                    if let Err(e) = result {
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
