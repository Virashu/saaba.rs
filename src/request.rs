use crate::HTTPMethod;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: HTTPMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn new() -> Self {
        Request {
            method: HTTPMethod::GET,
            path: String::from(""),
            headers: HashMap::new(),
        }
    }
}
