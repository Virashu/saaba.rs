use std::{
    collections::HashMap,
    fs,
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
};

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
        HTTPMethodLike: TryInto<HTTPMethod> + std::fmt::Debug,
        HandlerFunctionLike: Fn(Request) -> Response + 'static,
        <HTTPMethodLike as TryInto<HTTPMethod>>::Error: std::fmt::Display,
    {
        let boxed_cb = Box::new(callback);
        let http_method: HTTPMethod = method
            .try_into()
            .inspect_err(|e| log::error!("{}", e))
            .unwrap_or_default();

        let key: (HTTPMethod, String) = (http_method, url.to_owned());

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
        let mut url_ = url.to_string();

        url_ = url_.trim_end_matches('/').to_string();

        if !url_.starts_with('/') {
            url_ = format!("/{url_}");
        }

        self.static_handlers.insert(url_, dest.to_string());
        self
    }

    pub fn run(&self, hostname: &str, port: u32) -> Result<(), io::Error> {
        let addr = format!("{hostname}:{port}");
        let listener = TcpListener::bind(addr)?;

        for stream in listener.incoming() {
            let stream = stream?;

            self.handle_connection(stream);
        }

        Ok(())
    }

    fn try_find_exact(&self, request: &Request) -> Option<Response> {
        let method = request.method;
        let url = request.url.clone();

        let handler_option: Option<&ExactHandler> = self.exact_handlers.get(&(method, url));

        handler_option.map(|handler| handler(request.clone()))
    }

    fn try_find_var(&self, request: &Request) -> Option<Response> {
        let url = request.url.clone();

        let re_replacement = regex::Regex::new(r"\{(?<v>\w+)\}").unwrap();

        for k in self.var_handlers.keys() {
            let re_string_semi = &k.1; // semi regex expression
            let re_string = re_replacement
                .replace_all(re_string_semi, r"(?<$v>\w+)")
                .into_owned();

            let re_url = regex::Regex::new(&re_string).unwrap();

            if re_url.is_match(&url) {
                log::debug!("Found var handler: {}", re_string_semi);

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

    fn url_starts_with(url: String, key: String) -> bool {
        let url_seg = url.split("/").collect::<Vec<_>>();
        let key_seg = key.split("/").collect::<Vec<_>>();
        url_seg.starts_with(&key_seg) || key == "/"
    }

    fn similarity(url: String, key: String) -> i32 {
        let url_seg = url.split("/");
        let key_seg = key.split("/");

        url_seg.zip(key_seg).take_while(|(u, k)| u == k).count() as i32
    }

    fn try_find_static(&self, request: &Request) -> Option<Response> {
        let mut url = request.url.clone();
        let keys = self.static_handlers.keys();

        // Find keys for current path
        let mut keys: Vec<&String> = keys
            .filter(|k| Self::url_starts_with(url.clone(), k.to_string()))
            .collect();

        if keys.is_empty() {
            return None;
        }

        keys.sort_by_key(|k| Self::similarity(url.to_string(), k.to_string()));

        let selected = keys.last().unwrap().to_string();

        if selected == "/" {
            url.insert(0, '/');
        }

        let dest_option: Option<&String> = self.static_handlers.get(&selected);

        if let Some(dest) = dest_option {
            let resource_path_string = url.replacen(&selected, dest, 1);
            let resource_path = Path::new(&resource_path_string);
            log::debug!(
                "Found static resource path `{}` on handler `{}`",
                resource_path_string,
                selected
            );

            if !resource_path.exists() {
                return None;
            }

            if resource_path.is_file() {
                return Self::try_read_file(resource_path_string);
            }

            if resource_path.is_dir() {
                if !resource_path_string.ends_with("/") {
                    log::debug!("Adding a slash to `{}` as it is a path", request.url);
                    return Some(Response::redirect(request.url.clone() + "/"));
                }

                let resource_index_string = resource_path_string.clone() + "index.html";
                let resource_index_path = Path::new(&resource_index_string);

                if resource_index_path.exists() && resource_index_path.is_file() {
                    return Self::try_read_file(resource_index_string);
                }
            }
        }

        None
    }

    fn try_read_file(file_path_string: String) -> Option<Response> {
        //! file_path_string: path of **existing** file

        let file_path = Path::new(&file_path_string);

        match fs::read(file_path) {
            Ok(content) => {
                let type_ = guess_mime(&file_path_string);
                let mut res = Response::from_content_bytevec(content);

                if let Some(t) = type_ {
                    res.set_header(Header::ContentType, &t)
                }

                Some(res)
            }
            Err(_) => {
                log::error!("Cannot read file `{}`", file_path_string);
                None
            }
        }
    }

    fn find_response(&self, request: Request) -> Response {
        log::debug!("Seeking for handler: {}", &request.url);

        self.try_find_exact(&request)
            .or_else(|| self.try_find_var(&request))
            .or_else(|| self.try_find_static(&request))
            .unwrap_or(Response::not_found())
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        if http_request.is_empty() {
            return;
        }

        // Main header
        let request_v = http_request[0].split(' ').collect::<Vec<&str>>();
        let url = request_v[1].to_string();

        let method_str = request_v[0];

        let method = HTTPMethod::try_from(method_str)
            .inspect_err(|e| log::error!("{}", e))
            .unwrap_or_default();

        let headers = parse_headers(http_request[1..].to_vec());

        let request = Request {
            method,
            url: url.clone(),
            headers,
        };

        log::debug!("Request: {request:#?}");

        let response = self.find_response(request);

        stream.write_all(response.build().as_slice()).unwrap();
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}
