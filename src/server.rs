use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
// usize std::convert::TryInto;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }
}

impl Server {
    pub fn run(self) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // dbg!(&stream);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            // Request::try_from(&buffer as &[u8]);
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse a request {} ", e),
                            }
                        },
                        Err(e) => println!("Failed to read from connection {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection {}", e),
            }
        }
    }
}
