use std::fs;
use std::path::Path;
use std::io::prelude::*;
use std::net::TcpListener;

use urlencoding;



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

        let contents = match fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(_) => {
                send500(response);
                return;
            }
        };
            
        response.status = HTTPResponseStatus::OK.to_string();

        let contents = contents.replace("\u{0}", "");

        
        response.body = contents.clone();
        response.headers.push("Content-Type: application/octet-stream".to_string());
        response.headers.push("Content-Length: ".to_owned()+ &contents.len().to_string());
        response.send();

    });

    routes.post("/files/:filename", |request, response| {
        fn send500(response: &mut Response) {
            response.status = "500 Internal Server Error".to_string();
            response.body = "500 Internal Server Error".to_string();
            response.headers.push("Content-Type: text/plain".to_string());
            response.headers.push("Content-Length: 21".to_string());
            response.send();
        }

        let filename = match request.params.get("filename") {
            Some(f) => f,
            None => {
                send500(response);
                return;
            }
        };

        let env_args: Vec<String> = std::env::args().collect();
        let dir = match env_args.get(2) {
            Some(d) => d.clone(),
            None => {
                send500(response);
                return;
            }
        };

        let decoded_filename = match urlencoding::decode(filename) {
            Ok(df) => df.to_string(),
            Err(_) => {
                send500(response);
                return;
            }
        };

        let path = format!("{}/{}", dir, decoded_filename);

        if !Path::new(&dir).exists() {
            response.status = "404 Not Found".to_string();
            response.body = "404 Not Found".to_string();
            response.headers.push("Content-Type: text/plain".to_string());
            response.headers.push("Content-Length: 13".to_string());
            response.send();
            return;
        }

        let body =  request.body.clone();
        let body = body.replace("\u{0}", "");

        match fs::write(&path, &body) {
            Ok(_) => {
                println!("File written to {}", path);
                response.status = "201 Created".to_string();
                response.body = "201 Created".to_string();
                response.headers.push("Content-Type: text/plain".to_string());
                response.headers.push("Content-Length: 11".to_string());
                response.send();
            }
            Err(_) => {
                send500(response);
                return;
            }
        }


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





