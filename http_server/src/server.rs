use crate::http::{ParseError, Request, Response, StatusCode};
use std::{io::Read, net::TcpListener};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, error: &ParseError) -> Response {
        println!("Failed to parse request: {}", error);
        Response::new(StatusCode::BadRequest, None)
    }
}

// Everything is private by default
// Add pub keyword to make it public
pub struct Server {
    addr: String,
}

impl Server {
    // It is not a method but an associated function because
    // we call it without the need of an instance
    pub fn new(addr: String) -> Self {
        // Self is an alias to Server. Each struct as it
        Self {
            // if addr is also the name of the field then we can
            // just put its name, no need of 'addr: addr'
            addr,
        }
    }

    // a method
    // ownership of self is the same as any variable
    // As we never return here we can take the ownership, no need
    // to borrow it.
    // => run takes the ownership of self
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        // For infinite loop we can use "loop" keyword
        // It is possible to add label to loop to allow a break or continue from inner loop
        loop {
            // block thread until connection occurs
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buf = [0; 1024];

                    println!("new client from {}", addr);
                    match stream.read(&mut buf) {
                        Ok(n) => {
                            println!("Read {} bytes", n);

                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read data: {}", e),
                    }
                }
                Err(e) => println!("Connection failed with error {}", e),
            }
        }
    }
}
