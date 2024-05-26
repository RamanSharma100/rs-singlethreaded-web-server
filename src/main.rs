use std::io::prelude::*;
use std::net::TcpListener;

mod routes;
mod request;
mod response;

use routes::Routes;
use request::Request;
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

fn setup_routes() -> Routes {
    let mut routes = Routes::new();

    routes.get("/", |_, response| {
        response.status = HTTPResponseStatus::OK.to_string();
        response.body = "<h1>Hello, World!</h1>".to_string();
        response.headers.push("Content-Type: text/html".to_string());
        response.send();
    });

    // routes.get("/:name", |request, response| {
    //     response.status = HTTPResponseStatus::OK.to_string();
    //     response.body = format!("<h1>Hello, {}!</h1>", request.params.get("name").unwrap());
    //     response.headers.push("Content-Type: text/html".to_string());
    //     response.send();
    // });

    routes.get("/echo/:name", |request, response| {
        response.status = HTTPResponseStatus::OK.to_string();
        response.body = format!("{}", request.params.get("name").unwrap());
        response.headers.push("Content-Type: text/plain".to_string());
        response.headers.push("Content-Length: ".to_owned()+ &request.params.get("name").unwrap().len().to_string());
        response.send();
    });

    routes
}

pub fn handle_connection(stream: &mut std::net::TcpStream) {

    
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = Request::new(&String::from_utf8_lossy(&buffer[..]));
    
    let mut routes = setup_routes();   

    match routes.resolve(request.get_method(), &request.path) {
        Some(resolved) => {
            let (handler, params) = resolved;
            let mut response = Response::new(stream);
            let request_with_params = request.with_params(params);
            handler(request_with_params, &mut response);
        }
        None => {
            let mut response = Response::new(stream);
            response.status = HTTPResponseStatus::NOTFOUND.to_string();
            response.body = "404 Not Found".to_string();
            response.headers.push("Content-Type: text/plain".to_string());
            response.headers.push("Content-Length: 13".to_string());
            response.send();
            response.send(); 
        }
    }

}





