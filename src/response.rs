use std::fmt;
use std::io::prelude::*;

#[allow(dead_code)]
pub enum HTTPResponseStatus {
    OK = 200,
    CREATED = 201,
    ACCEPTED = 202,
    NOCONTENT = 204,
    MOVEDPERMANENTLY = 301,
    FOUND = 302,
    SEEOTHER = 303,
    NOTMODIFIED = 304,
    TEMPORARYREDIRECT = 307,
    PERMANENTREDIRECT = 308,
    BADREQUEST = 400,
    UNAUTHORIZED = 401,
    FORBIDDEN = 403,
    NOTFOUND = 404,
    METHODNOTALLOWED = 405,
    REQUESTTIMEOUT = 408,
    CONFLICT = 409,
    GONE = 410,
    LENGTHREQUIRED = 411,
    PAYLOADTOOLARGE = 413,
    URITOOLONG = 414,
    UNSUPPORTEDMEDIATYPE = 415,
    EXPECTATIONFAILED = 417,
    UPGRADEREQUIRED = 426,
    INTERNALSERVERERROR = 500,
    NOTIMPLEMENTED = 501,
    BADGATEWAY = 502,
}

impl fmt::Display for HTTPResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HTTPResponseStatus::OK => write!(f, "200 OK"),
            HTTPResponseStatus::CREATED => write!(f, "201 Created"),
            HTTPResponseStatus::ACCEPTED => write!(f, "202 Accepted"),
            HTTPResponseStatus::NOCONTENT => write!(f, "204 No Content"),
            HTTPResponseStatus::MOVEDPERMANENTLY => write!(f, "301 Moved Permanently"),
            HTTPResponseStatus::FOUND => write!(f, "302 Found"),
            HTTPResponseStatus::SEEOTHER => write!(f, "303 See Other"),
            HTTPResponseStatus::NOTMODIFIED => write!(f, "304 Not Modified"),
            HTTPResponseStatus::TEMPORARYREDIRECT => write!(f, "307 Temporary Redirect"),
            HTTPResponseStatus::PERMANENTREDIRECT => write!(f, "308 Permanent Redirect"),
            HTTPResponseStatus::BADREQUEST => write!(f, "400 Bad Request"),
            HTTPResponseStatus::UNAUTHORIZED => write!(f, "401 Unauthorized"),
            HTTPResponseStatus::FORBIDDEN => write!(f, "403 Forbidden"),
            HTTPResponseStatus::NOTFOUND => write!(f, "404 Not Found"),
            HTTPResponseStatus::METHODNOTALLOWED => write!(f, "405 Method Not Allowed"),
            HTTPResponseStatus::REQUESTTIMEOUT => write!(f, "408 Request Timeout"),
            HTTPResponseStatus::CONFLICT => write!(f, "409 Conflict"),
            HTTPResponseStatus::GONE => write!(f, "410 Gone"),
            HTTPResponseStatus::LENGTHREQUIRED => write!(f, "411 Length Required"),
            HTTPResponseStatus::PAYLOADTOOLARGE => write!(f, "413 Payload Too Large"),
            HTTPResponseStatus::URITOOLONG => write!(f, "414 URI Too Long"),
            HTTPResponseStatus::UNSUPPORTEDMEDIATYPE => write!(f, "415 Unsupported Media Type"),
            HTTPResponseStatus::EXPECTATIONFAILED => write!(f, "417 Expectation Failed"),
            HTTPResponseStatus::UPGRADEREQUIRED => write!(f, "426 Upgrade Required"),
            HTTPResponseStatus::INTERNALSERVERERROR => write!(f, "500 Internal Server Error"),
            HTTPResponseStatus::NOTIMPLEMENTED => write!(f, "501 Not Implemented"),
            HTTPResponseStatus::BADGATEWAY => write!(f, "502 Bad Gateway"),
        }
    }
}

pub enum Body {
    Text(String),
    Binary(Vec<u8>),
}

pub struct Response {
    pub status: String,
    pub body: Body,
    pub headers: Vec<String>,
    pub stream: std::net::TcpStream,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Response: status: {}, body: {}",
            self.status,
            match &self.body {
                Body::Text(body) => body,
                Body::Binary(_) => "Binary data",
            }
        )
    }
}

impl Response {
    pub fn new(stream: &mut std::net::TcpStream) -> Response {
        Response {
            headers: Vec::new(),
            body: Body::Text("".to_string()),
            status: "200 OK".to_string(),
            stream: stream.try_clone().unwrap(),
        }
    }

    pub fn send(&mut self) {
        let response = format!(
            "HTTP/1.1 {}\r\n{}\r\n\r\n{}",
            self.status,
            self.headers.join("\r\n"),
            match &self.body {
                Body::Text(body) => body.clone(),
                Body::Binary(data) => {
                    let body: String = data.iter().map(|&byte| byte as char).collect();
                    body
                }
            }
        );
        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn send_binary(&mut self) {
        // let headers_bytes: Vec<u8> = self.headers.join("\r\n").into_bytes();
        // let response = format!(
        //     "HTTP/1.1 {}\r\n{}\r\n\r\n{}",
        //     self.status,
        //     String::from_utf8_lossy(&headers_bytes), // Convert headers bytes back to string
        //     match &self.body {
        //         Body::Text(body) => body.clone(),
        //         Body::Binary(data) => {
        //             let body: String = data.iter().map(|&byte| byte as char).collect();
        //             body
        //         }
        //     }
        // );
        // self.stream.write(response.as_bytes()).unwrap();
        // self.stream.flush().unwrap();

        let mut response = [
            self.status.clone(),
            self.headers.join("\r\n"),
            "\r\n\r\n".to_string(),
        ]
        .join("\r\n")
        .into_bytes();
        response.extend(match &self.body {
            Body::Text(body) => body.clone().into_bytes(),
            Body::Binary(data) => data.clone(),
        });
        self.stream.write("HTTP/1.1 ".as_bytes()).unwrap();

        self.stream.write(&response).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn get_status_line(status: HTTPResponseStatus) -> String {
        match status {
            HTTPResponseStatus::OK => "200 OK".to_string(),
            HTTPResponseStatus::CREATED => "201 Created".to_string(),
            HTTPResponseStatus::ACCEPTED => "202 Accepted".to_string(),
            HTTPResponseStatus::NOCONTENT => "204 No Content".to_string(),
            HTTPResponseStatus::MOVEDPERMANENTLY => "301 Moved Permanently".to_string(),
            HTTPResponseStatus::FOUND => "302 Found".to_string(),
            HTTPResponseStatus::SEEOTHER => "303 See Other".to_string(),
            HTTPResponseStatus::NOTMODIFIED => "304 Not Modified".to_string(),
            HTTPResponseStatus::TEMPORARYREDIRECT => "307 Temporary Redirect".to_string(),
            HTTPResponseStatus::PERMANENTREDIRECT => "308 Permanent Redirect".to_string(),
            HTTPResponseStatus::BADREQUEST => "400 Bad Request".to_string(),
            HTTPResponseStatus::UNAUTHORIZED => "401 Unauthorized".to_string(),
            HTTPResponseStatus::FORBIDDEN => "403 Forbidden".to_string(),
            HTTPResponseStatus::NOTFOUND => "404 Not Found".to_string(),
            HTTPResponseStatus::METHODNOTALLOWED => "405 Method Not Allowed".to_string(),
            HTTPResponseStatus::REQUESTTIMEOUT => "408 Request Timeout".to_string(),
            HTTPResponseStatus::CONFLICT => "409 Conflict".to_string(),
            HTTPResponseStatus::GONE => "410 Gone".to_string(),
            HTTPResponseStatus::LENGTHREQUIRED => "411 Length Required".to_string(),
            HTTPResponseStatus::PAYLOADTOOLARGE => "413 Payload Too Large".to_string(),
            HTTPResponseStatus::URITOOLONG => "414 URI Too Long".to_string(),
            HTTPResponseStatus::UNSUPPORTEDMEDIATYPE => "415 Unsupported Media Type".to_string(),
            HTTPResponseStatus::EXPECTATIONFAILED => "417 Expectation Failed".to_string(),
            HTTPResponseStatus::UPGRADEREQUIRED => "426 Upgrade Required".to_string(),
            HTTPResponseStatus::INTERNALSERVERERROR => "500 Internal Server Error".to_string(),
            HTTPResponseStatus::NOTIMPLEMENTED => "501 Not Implemented".to_string(),
            HTTPResponseStatus::BADGATEWAY => "502 Bad Gateway".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn set_status(&mut self, status: HTTPResponseStatus) -> &mut Self {
        self.status = Response::get_status_line(status);
        return self;
    }

    #[allow(dead_code)]
    pub fn set_body(&mut self, body: Body) -> &mut Self {
        self.body = body;
        return self;
    }

    #[allow(dead_code)]
    pub fn add_header(&mut self, header: String) -> &mut Self {
        self.headers.push(header);
        return self;
    }
}
