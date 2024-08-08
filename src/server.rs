use std::io::{Read, Write};
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::{Response, Request, StatusCode, ParseError};
use std::net::TcpListener;


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}



pub struct Server {
    addr: String,
}

impl Server {
    // associate function
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    // method, have "self"
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening to {}", self.addr);
        let listener = TcpListener::bind(& self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // let arr = [1, 2, 3, 4];
                    // funa(&arr[1..2]);
                    let mut buf = [0; 1024];
                    
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Receive a request: {}", String::from_utf8_lossy(&buf));
                            
                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                },
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }
        }

        
    }
}