extern crate tiny_http;
use tiny_http::{Response, Server, ServerConfig};
fn main() {
    let server = Server::http("0.0.0.0:").unwrap();

    loop {
        let request = match server.recv() {
            Ok(req) => println!("{:?}", req),
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        };
    }
}
