use super::{HTTPMethod, Request, Response};
use regex::Regex;
use std::collections::HashMap;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

// type ResponseValue = impl Into<Response>;

type HandlerFunction = fn(Request) -> Response;

pub struct App {
    handlers: HashMap<(HTTPMethod, String), HandlerFunction>,
}

fn parse_headers(headers_vec: Vec<String>) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    let header_re = Regex::new(r"(?<key>[a-zA-Z-_]+):\s?(?<value>.+)").unwrap();

    for line in headers_vec.into_iter() {
        let rm = header_re.captures(line.as_str()).unwrap();
        headers.insert(rm["key"].into(), rm["value"].into());
    }

    headers
}

impl App {
    pub fn new() -> App {
        App {
            handlers: HashMap::new(),
        }
    }

    pub fn route<H: Sized>(&mut self, method: H, path: &str, callback: HandlerFunction)
    where
        H: Into<HTTPMethod>,
    {
        self.handlers
            .insert((method.into(), path.to_owned()), callback);
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

        // Main header
        let request_v = http_request[0].split(' ').collect::<Vec<&str>>();
        let path = request_v[1].to_string();
        let method = HTTPMethod::from(request_v[0].to_string());

        let headers = parse_headers(http_request[1..].to_vec());

        let request = Request {
            headers,
            path: path.clone(),
            method: method.clone(),
        };

        let handler_option: Option<&HandlerFunction> = self.handlers.get(&(method, path));

        let response: Response;

        if let Some(handler) = handler_option {
            response = Response::from(handler(request));
        } else {
            response = Response::not_found();
        }

        stream.write_all(response.build().as_slice()).unwrap();
    }
}
