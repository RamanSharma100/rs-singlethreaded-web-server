use std::io::prelude::*;



pub struct Route {
    pub method: Method,
    pub path: String,
    pub handler: fn(&mut std::net::TcpStream),
}

