#![allow(dead_code)]

use server::Server;
use website_handler::WebSiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    // We want to pass the IP and the port
    let server = Server::new("127.0.0.1:8080".to_string());

    // will run forever and wait for request
    server.run(WebSiteHandler);
}

/* sample of a request
 * -----------------------------
 * GET /user?id=10 HTTP/1.1\r\n
 * HEADERS \r\n
 * BODY
 */
