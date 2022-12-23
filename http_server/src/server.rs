use std::{net::TcpListener, io::Read};

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
    pub fn run(self) {
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
                    stream.read(&mut buf).unwrap();

                    println!("=== BUFFER ===");
                    println!("{:?}", buf);
                }
                Err(e) => println!("Connection failed with error {}", e),
            }
        }
    }
}
