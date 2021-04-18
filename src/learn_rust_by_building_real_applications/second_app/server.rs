use crate::second_app::http::Request;
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

                    match Request::try_from(&buffer[0..=num]) {
                        Ok(request) => {
                            dbg!(request);
                        }
                        Err(_) => {}
                    }
                }
                Err(e) => {
                    println!("Failed to read from connection: {}", e);
                }
            }
        }
    }
}
