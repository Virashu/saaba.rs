use crate::response;

use super::{HTTPMethod, Request, Response};
use std::collections::HashMap;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

type HandlerFunction = fn(Request) -> Response;
const CRLF: &str = "\r\n";

pub struct App {
    handlers: HashMap<(HTTPMethod, String), HandlerFunction>,
}

impl App {
    pub fn new() -> App {
        App {
            handlers: HashMap::new(),
        }
    }

    pub fn route(&mut self, method: HTTPMethod, path: &str, callback: HandlerFunction) {
        self.handlers.insert((method, path.to_owned()), callback);
    }

    pub fn get(&mut self, path: &str, callback: HandlerFunction) {
        self.route(HTTPMethod::GET, path, callback)
    }

    pub fn post(&mut self, path: &str, callback: HandlerFunction) {
        self.route(HTTPMethod::POST, path, callback)
    }

    pub fn run(&mut self, hostname: &str, port: u32) {
        let addr = format!("{hostname}:{port}");
        let listener = TcpListener::bind(addr).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.handle_connection(stream);
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {http_request:#?}");

        if http_request.len() == 0 {
            return;
        }

        let request_v = http_request[0].split(' ').collect::<Vec<&str>>();

        let method = HTTPMethod::from(request_v[0].to_string());
        let path = request_v[1].to_string();

        let request = Request {
            headers: Vec::new(), // TODO
            path: path.clone(),
            method: method.clone(),
        };

        let handler = self.handlers.get(&(method, path)).unwrap();
        let response = Response::from(handler(request));
        let response_content = response.content;
        let headers_string = [response.headers, vec![String::from("Content-Type: ") + &response.content_type]].concat().join(CRLF);
        let status = 200;
        let status_text = "OK";
        let full_response_text = format!("HTTP/1.1 {status} {status_text}{CRLF}{headers_string}{CRLF}{CRLF}{response_content}");

        stream.write_all(full_response_text.as_bytes()).unwrap();
    }
}
