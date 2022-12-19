use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    // We want to pass the IP and the port
    let server = Server::new("127.0.0.1:8080".to_string());

    // will run forever and wait for request
    server.run();
}

/* sample of a request
 * -----------------------------
 * GET /user?id=10 HTTP/1.1\r\n
 * HEADERS \r\n
 * BODY
 */
