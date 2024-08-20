use std::{
    collections::HashMap,
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use log::debug;

use super::utils::parse_headers;
use super::{HTTPMethod, Request, Response};

pub struct App {
    handlers: HashMap<(HTTPMethod, String), Box<dyn Fn(Request) -> Response + 'static>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn route<HTTPMethodLike, HandlerFunctionLike>(
        &mut self,
        method: HTTPMethodLike,
        path: &str,
        callback: HandlerFunctionLike,
    ) -> &mut Self
    where
        HTTPMethodLike: Into<HTTPMethod>,
        HandlerFunctionLike: Fn(Request) -> Response + 'static,
    {
        let boxed_cb = Box::new(callback);
        self.handlers
            .insert((method.into(), path.to_owned()), boxed_cb);
        self
    }

    pub fn get<HandlerFunctionLike>(
        &mut self,
        path: &str,
        callback: HandlerFunctionLike,
    ) -> &mut Self
    where
        HandlerFunctionLike: Fn(Request) -> Response + 'static,
    {
        self.route(HTTPMethod::GET, path, callback)
    }

    pub fn post<HandlerFunctionLike>(
        &mut self,
        path: &str,
        callback: HandlerFunctionLike,
    ) -> &mut Self
    where
        HandlerFunctionLike: Fn(Request) -> Response + 'static,
    {
        self.route(HTTPMethod::POST, path, callback)
    }

    pub fn run(&mut self, hostname: &str, port: u32) -> Result<(), io::Error> {
        let addr = format!("{hostname}:{port}");
        let listener = TcpListener::bind(addr)?;

        for stream in listener.incoming() {
            let stream = stream?;

            self.handle_connection(stream);
        }

        Ok(())
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        if http_request.len() == 0 {
            return;
        }

        // Main header
        let request_v = http_request[0].split(' ').collect::<Vec<&str>>();
        let path = request_v[1].to_string();
        let method = HTTPMethod::from(request_v[0].to_string());

        let headers = parse_headers(http_request[1..].to_vec());

        let request = Request {
            method: method.clone(),
            path: path.clone(),
            headers,
        };

        debug!("Request: {request:#?}");

        let handler_option: Option<&Box<dyn Fn(Request) -> Response>> = self.handlers.get(&(method, path));

        let response: Response;

        if let Some(boxed_handler_ref) = handler_option {
            response = Response::from((**boxed_handler_ref)(request));
        } else {
            response = Response::not_found();
        }

        stream.write_all(response.build().as_slice()).unwrap();
    }
}
