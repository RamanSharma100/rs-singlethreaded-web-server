use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;

mod routes;
mod request;
mod response;
mod encoding;

use routes::Routes;
use request::Request;
use encoding::Encoding;
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

    routes.get("/user-agent", |request,response| {
        let user_agent = request.read_header("User-Agent").unwrap();

        response.status = HTTPResponseStatus::OK.to_string();
        response.body = format!("{}", user_agent);
        response.headers.push("Content-Type: text/plain".to_string());
        response.headers.push("Content-Length: ".to_owned()+ &user_agent.len().to_string());
        response.send();
    }); 

    routes.get("/files/:filename", |request,response|{

        fn send500(response: &mut Response) {
            response.status = HTTPResponseStatus::INTERNALSERVERERROR.to_string();
            response.body = "500 Internal Server Error".to_string();
            response.headers.push("Content-Type: text/plain".to_string());
            response.headers.push("Content-Length: 21".to_string());
            response.send();
        }


        let filename = request.params.get("filename").unwrap();
        let env_args = std::env::args().collect::<Vec<String>>();
        let dir = env_args.get(2).cloned().unwrap_or_else(|| {
            send500(response);
            return "".to_string();
        });
        let path = format!("{}/{}", dir, Encoding::precentage_decode(filename));

        let binding = filename.split(".").collect::<Vec<&str>>();
        let file_ext = binding.last().unwrap();

        let content_type = match file_ext {
            &"html" => "text/html",
            &"css" => "text/css",
            &"js" => "text/javascript",
            &"png" => "image/png",
            &"jpg" => "image/jpeg",
            &"jpeg" => "image/jpeg",
            &"gif" => "image/gif",
            &"svg" => "image/svg+xml",
            &"ico" => "image/x-icon",
            &"json" => "application/json",
            &"pdf" => "application/pdf",
            _ => "application/octet-stream",
        };

        let isBinary = match file_ext {
            &"png" => true,
            &"jpg" => true,
            &"jpeg" => true,
            &"gif" => true,
            &"svg" => true,
            &"ico" => true,
            &"pdf" => true,
            _ => false,
        };


        if !fs::metadata(&path).is_ok() {
            response.status = HTTPResponseStatus::NOTFOUND.to_string();
            response.body = "404 Not Found".to_string();
            response.headers.push("Content-Type: text/plain".to_string());
            response.headers.push("Content-Length: 13".to_string());
            response.send();
            return;
        }

        if fs::metadata(&path).unwrap().is_dir() {
            response.status = HTTPResponseStatus::FORBIDDEN.to_string();
            response.body = "403 Forbidden".to_string();
            response.headers.push("Content-Type: text/plain".to_string());
            response.headers.push("Content-Length: 13".to_string());
            response.send();
            return;
        }


        

        let contents = match fs::read(&path) {
            Ok(contents) => contents,
            Err(_) => {
                send500(response);
                return;
            }
        };

        response.status = HTTPResponseStatus::OK.to_string();
        response.body = if isBinary {
            Encoding::base64_encode(
                &String::from_utf8_lossy(&contents).to_string()
            )
        } else {
            String::from_utf8_lossy(&contents).to_string()
        };
        response.headers.push("Content-Type: ".to_owned() + content_type);
        response.headers.push("Content-Length: ".to_owned() + &contents.len().to_string());
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





