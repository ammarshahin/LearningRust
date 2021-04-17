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
            let (mut stream, _) = match listener.accept() {
                Ok(tub) => tub,
                Err(e) => panic!("error establish a connection: {}", e),
            };

            let mut buffer = [0; 1024];

            match stream.read(&mut buffer) {
                Ok(num) => {
                    println!(
                        "received a request of {}bytes: {}",
                        num,
                        String::from_utf8_lossy(&buffer)
                    );
                }
                Err(e) => {
                    println!("Failed to read from connection: {}", e);
                }
            }
        }
    }
}
