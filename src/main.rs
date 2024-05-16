use std::net::TcpListener;

mod response;

fn main() {
    println!("Started Server on http://127.0.0.1:4221");

   
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                response::handle_connection(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
