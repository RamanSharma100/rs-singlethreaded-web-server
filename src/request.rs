use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub enum HTTPRequestMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    UNKNOWN,
}

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE,
    OTHER,
}

#[derive(Debug)]

pub struct Request {
    pub method: Method,
    pub path: String,
    pub params: HashMap<String, String>,
    pub query_string: String,
    pub body: String,
    pub headers: Vec<String>,
}

impl Request {

    pub fn new(request: &str) -> Request {
        let mut headers = Vec::new();
        let mut body = String::new();
        let mut method = Method::OTHER;
        let _params: HashMap<String,String> = HashMap::new();
        let mut path = String::new();
        let mut query_string = String::new();
        let mut is_body = false;
        for (i, line) in request.lines().enumerate() {
            if i == 0 {
                let parts: Vec<&str> = line.split(" ").collect();
                method = match parts[0] {
                    "GET" => Method::GET,
                    "POST" => Method::POST,
                    "PUT" => Method::PUT,
                    "DELETE" => Method::DELETE,
                    "PATCH" => Method::PATCH,
                    "OPTIONS" => Method::OPTIONS,
                    "HEAD" => Method::HEAD,
                    "CONNECT" => Method::CONNECT,
                    "TRACE" => Method::TRACE,
                    _ => Method::OTHER,
                };
                let path_parts: Vec<&str> = parts[1].split("?").collect();
                path = path_parts[0].to_string();
                if path_parts.len() > 1 {
                    query_string = path_parts[1].to_string();
                }
            } else {
                if line == "" {
                    is_body = true;
                } else if is_body {
                    body.push_str(line);
                    body.push_str("\n");
                } else {
                    headers.push(line.to_string());
                }
            }
        }

        Request {
            method,
            path,
            params: _params,
            query_string,
            body,
            headers,
        }
    }

    pub fn get_method(&self) -> HTTPRequestMethod {
        match self.method {
            Method::GET => HTTPRequestMethod::GET,
            Method::POST => HTTPRequestMethod::POST,
            Method::PUT => HTTPRequestMethod::PUT,
            Method::DELETE => HTTPRequestMethod::DELETE,
            Method::PATCH => HTTPRequestMethod::PATCH,
            Method::OPTIONS => HTTPRequestMethod::OPTIONS,
            Method::HEAD => HTTPRequestMethod::HEAD,
            Method::CONNECT => HTTPRequestMethod::CONNECT,
            Method::TRACE => HTTPRequestMethod::TRACE,
            Method::OTHER => HTTPRequestMethod::UNKNOWN,
        }
    }

    pub fn with_params(mut self, params: HashMap<String, String>) -> Self {
        self.params = params;
        self
    }

}