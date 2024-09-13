use std::{
    collections::HashMap,
    fs,
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
};

use log::debug;

use crate::mime::guess_mime;

use super::header::Header;
use super::utils::parse_headers;
use super::{HTTPMethod, Request, Response};

type HandlerKey = (HTTPMethod, String);
type ExactHandler = Box<dyn Fn(Request) -> Response + 'static>;
type VarHandler = Box<dyn Fn(Request, HashMap<&str, &str>) -> Response + 'static>;

pub struct App {
    exact_handlers: HashMap<HandlerKey, ExactHandler>,
    var_handlers: HashMap<HandlerKey, VarHandler>,
    static_handlers: HashMap<String, String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            exact_handlers: HashMap::new(),
            var_handlers: HashMap::new(),
            static_handlers: HashMap::new(),
        }
    }

    pub fn route<HTTPMethodLike, HandlerFunctionLike>(
        &mut self,
        method: HTTPMethodLike,
        url: &str,
        callback: HandlerFunctionLike,
    ) -> &mut Self
    where
        HTTPMethodLike: Into<HTTPMethod>,
        HandlerFunctionLike: Fn(Request) -> Response + 'static,
    {
        let boxed_cb = Box::new(callback);
        let key: (HTTPMethod, String) = (method.into(), url.to_owned());

        self.exact_handlers.insert(key, boxed_cb);

        self
    }

    pub fn route_var<HTTPMethodLike, VarHandlerFunctionLike>(
        &mut self,
        method: HTTPMethodLike,
        url: &str,
        callback: VarHandlerFunctionLike,
    ) -> &mut Self
    where
        HTTPMethodLike: Into<HTTPMethod>,
        VarHandlerFunctionLike: Fn(Request, HashMap<&str, &str>) -> Response + 'static,
    {
        let boxed_cb = Box::new(callback);
        let key: (HTTPMethod, String) = (method.into(), url.to_owned());

        self.var_handlers.insert(key, boxed_cb);

        self
    }

    // Shorthands

    pub fn get<HandlerFunctionLike>(
        &mut self,
        url: &str,
        callback: HandlerFunctionLike,
    ) -> &mut Self
    where
        HandlerFunctionLike: Fn(Request) -> Response + 'static,
    {
        self.route(HTTPMethod::GET, url, callback)
    }

    pub fn post<HandlerFunctionLike>(
        &mut self,
        url: &str,
        callback: HandlerFunctionLike,
    ) -> &mut Self
    where
        HandlerFunctionLike: Fn(Request) -> Response + 'static,
    {
        self.route(HTTPMethod::POST, url, callback)
    }

    pub fn get_var<VarHandlerFunctionLike>(
        &mut self,
        url: &str,
        callback: VarHandlerFunctionLike,
    ) -> &mut Self
    where
        VarHandlerFunctionLike: Fn(Request, HashMap<&str, &str>) -> Response + 'static,
    {
        self.route_var(HTTPMethod::GET, url, callback)
    }

    pub fn post_var<VarHandlerFunctionLike>(
        &mut self,
        url: &str,
        callback: VarHandlerFunctionLike,
    ) -> &mut Self
    where
        VarHandlerFunctionLike: Fn(Request, HashMap<&str, &str>) -> Response + 'static,
    {
        self.route_var(HTTPMethod::POST, url, callback)
    }

    pub fn static_(&mut self, url: &str, dest: &str) -> &mut Self {
        self.static_handlers
            .insert(url.to_string(), dest.to_string());
        self
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

    fn try_find_exact(&self, request: &Request) -> Option<Response> {
        let method = request.method.clone();
        let url = request.url.clone();

        let handler_option: Option<&ExactHandler> = self.exact_handlers.get(&(method, url));

        if let Some(handler) = handler_option {
            Some(handler(request.clone()))
        } else {
            None
        }
    }

    fn try_find_var(&self, request: &Request) -> Option<Response> {
        let url = request.url.clone();

        let re_replacement = regex::Regex::new(r"\{(?<v>\w+)\}").unwrap();

        for k in self.var_handlers.keys() {
            let re_string_semi = &k.1; // semi regex expression
            let re_string = re_replacement
                .replace_all(&re_string_semi, r"(?<$v>\w+)")
                .into_owned();

            let re_url = regex::Regex::new(&re_string).unwrap();

            if re_url.is_match(&url) {
                debug!("Found var handler: {}", re_string_semi);

                let cap = re_url.captures(&url).unwrap();

                let vars: HashMap<&str, &str> = re_url
                    .capture_names()
                    .flatten()
                    .filter_map(|n| Some((n, cap.name(n)?.as_str())))
                    .collect();

                let handler = self.var_handlers.get(k).unwrap();
                let res = handler(request.clone(), vars);
                return Some(res);
            }
        }
        None
    }

    fn try_find_static(&self, request: &Request) -> Option<Response> {
        let url = request.url.clone();
        let keys = self.static_handlers.keys();

        // Find keys for current path
        let mut keys: Vec<&String> = keys.filter(|k| url.starts_with(*k)).collect();

        if keys.len() == 0 {
            return None;
        }

        // Decide which key is the most accurate (specific)
        // `/app/home` > `/app`
        keys.sort_by(|a, b| {
            let matches_a: Vec<&str> = a.matches("/").collect();
            let len_a = matches_a.len();

            let matches_b: Vec<&str> = b.matches("/").collect();
            let len_b = matches_b.len();

            len_a.cmp(&len_b)
        });

        let selected = keys[0];

        let dest_option: Option<&String> = self.static_handlers.get(selected);

        if let Some(dest) = dest_option {
            let file_path_string = url.replace(selected, dest);
            let file_path = Path::new(&file_path_string);
            debug!("Found static resource path: {}", file_path_string);

            if file_path.exists() {
                return Self::try_read_file(file_path_string);
            }

            let file_path_string = file_path_string + "/index.html";
            let file_path = Path::new(&file_path_string);

            if file_path.exists() {
                return Self::try_read_file(file_path_string);
            }
        }

        None
    }

    fn try_read_file(file_path_string: String) -> Option<Response> {
        let file_path = Path::new(&file_path_string);

        return match fs::read(file_path) {
            Ok(content) => {
                let type_ = guess_mime(&file_path_string);
                let mut res = Response::from_content_bytevec(content);

                match type_ {
                    Some(t) => res.set_header(Header::ContentType, &t),
                    None => {}
                }

                Some(res)
            }
            Err(_) => None,
        };
    }

    fn find_response(&self, request: Request) -> Response {
        debug!("Seeking for handler: {}", &request.url);

        self.try_find_exact(&request)
            .or_else(|| self.try_find_var(&request))
            .or_else(|| self.try_find_static(&request))
            .unwrap_or(Response::not_found())
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
        let url = request_v[1].to_string();
        let method = HTTPMethod::from(request_v[0].to_string());

        let headers = parse_headers(http_request[1..].to_vec());

        let request = Request {
            method: method.clone(),
            url: url.clone(),
            headers,
        };

        debug!("Request: {request:#?}");

        let response = self.find_response(request);

        stream.write_all(response.build().as_slice()).unwrap();
    }
}
