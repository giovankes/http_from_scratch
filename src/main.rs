/* simple HTTP server */
/* author: Giovanni */

/* reasoning: understanding HTTP better */

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    /* creating a local tcplistener at port 8477 */
    const HOST: &str = "0.0.0.0";
    const PORT: &str = "8477";

    /* concat host address and port for final end point*/
    let end_point: String = HOST.to_owned() + ":" + PORT;

    /* create a tcp listener at our end point */
    let listener = TcpListener::bind(end_point).unwrap();

    /* let developer know that the web server is listening */
    println!("web server is listening at port {}", PORT);

    /* communicate with any incoming connections */

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        /*call function that handles the incoming connection (aka stream)*/
        handle_connection(_stream);
    }
}

/* function that actually handles the incoming connections */

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response_content = fs::read_to_string("index.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_content.len(),
        response_content
    );

    println!("request: {}", String::from_utf8_lossy(&buffer[..]));
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
