use std::net::TcpListener;
use std::io::prelude::*;

mod request;
mod routes;
mod response;

use request::Request;
use routes::Routes;
use response::{Response, HTTPResponseStatus};

fn main() {
    println!("Started Server on http://127.0.0.1:4221");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

   

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_connection(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

pub fn handle_connection(stream: &mut std::net::TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = Request::new(&String::from_utf8_lossy(&buffer[..]));
    
    let mut routes = Routes::new();

    let mut cloned_stream = stream.try_clone().expect("Failed to clone stream");

    Routes::get(&mut routes, "/", move |_, mut response| {
        response.status = HTTPResponseStatus::OK.to_string();
        response.send(&mut cloned_stream);
    });



    if let Some(handler) = routes.resolve(request.get_method(), &request.path) {
        let response = Response::new(&Response::get_status_line(HTTPResponseStatus::OK), &HTTPResponseStatus::OK.to_string());
        handler(request, response);
    } else {
        let response = Response::new(&Response::get_status_line(HTTPResponseStatus::NOTFOUND), &HTTPResponseStatus::NOTFOUND.to_string());
        response.send(stream);
    }
}
